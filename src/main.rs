//! arb_poc — 라이브 트레이딩 진입점.
//!
//! 실제 거래를 수행하는 라이브 모드의 메인 바이너리입니다.
//!
//! ## 사전 준비
//!
//! 1. MySQL DB 준비 + 마이그레이션 실행:
//!    ```bash
//!    DATABASE_URL=mysql://user:pass@localhost/arb cargo run --example migrate
//!    ```
//!
//! 2. 설정 파일: `config.toml` (API 키 + DB URL) + `strategy.toml` (전략 파라미터)
//!
//! ## 실행 방법
//!
//! ```bash
//! # config.toml에 모든 시크릿이 설정된 경우
//! cargo run
//!
//! # 디버그 로그
//! RUST_LOG=debug cargo run
//! ```
//!
//! ## Graceful Shutdown
//!
//! `Ctrl+C` (SIGINT) 또는 `SIGTERM`으로 graceful shutdown 합니다.
//! shutdown_policy 설정에 따라 포지션을 유지하거나 청산합니다.

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use tracing::{error, info, warn};

use arb_poc::adapter::DbPositionStoreAdapter;
use arb_poc::config::Config;
use arb_poc::db::alerts::AlertRepository;
use arb_poc::db::balance_snapshots::BalanceSnapshotRepository;
use arb_poc::db::funding::FundingRepository;
use arb_poc::db::minutes::MinuteRepository;
use arb_poc::db::pool::{DbPool, DbPoolConfig};
use arb_poc::db::positions::DbPositionStore;
use arb_poc::db::sessions::SessionRepository;
use arb_poc::db::trades::TradeRepository;
use arb_poc::db::writer::DbWriter;
use arb_poc::exchange::{ExchangeAdapter, MarketData, OrderManagement};
use arb_poc::exchanges::{BybitAdapter, BybitClient, UpbitAdapter, UpbitClient};
use arb_poc::forex::{ForexCache, UsdtKrwCache};
use arb_poc::strategy::zscore::balance::BalanceTracker;
use arb_poc::strategy::zscore::balance_recorder::BalanceRecorderTask;
use arb_poc::strategy::zscore::config::ZScoreConfig;
use arb_poc::strategy::zscore::live_executor::LiveExecutor;
use arb_poc::strategy::zscore::monitor::ZScoreMonitor;
use arb_poc::strategy::zscore::monitor_live::LivePolicy;
use arb_poc::strategy::zscore::pnl::ClosedPosition;
use arb_poc::strategy::zscore::risk::{RiskConfig, RiskManager};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 로깅 초기화
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    info!("=== arb_poc 라이브 트레이딩 시작 ===");

    // ---------------------------------------------------------------
    // 1. 설정 로드 + 검증
    // ---------------------------------------------------------------
    let config = Config::load_or_default();

    let strategy_config_path =
        std::env::var("STRATEGY_CONFIG").unwrap_or_else(|_| "strategy.toml".into());
    let strategy_config = if Path::new(&strategy_config_path).exists() {
        info!(path = %strategy_config_path, "전략 설정 파일 로드");
        let cfg = ZScoreConfig::from_file(&strategy_config_path)?;
        cfg.validate()?;
        cfg
    } else {
        return Err("strategy.toml 파일이 필요합니다. 라이브 모드에서는 기본값 사용 불가.".into());
    };

    info!(
        coins = ?strategy_config.coins,
        window = strategy_config.window_size,
        entry_z = strategy_config.entry_z_threshold,
        exit_z = strategy_config.exit_z_threshold,
        total_capital = %strategy_config.total_capital_usdt,
        "전략 설정 로드 완료"
    );

    // API 키 검증 (라이브 모드 필수)
    if !config.upbit.has_credentials() {
        return Err("Upbit API 키가 필요합니다. config.toml을 확인하세요.".into());
    }
    if !config.bybit.has_credentials() {
        return Err("Bybit API 키가 필요합니다. config.toml을 확인하세요.".into());
    }

    // ---------------------------------------------------------------
    // 2. DB 초기화
    // ---------------------------------------------------------------
    if !config.database.is_configured() {
        return Err(
            "DB URL이 필요합니다. config.toml의 [database] url 또는 DATABASE_URL 환경변수를 설정하세요."
                .into(),
        );
    }
    let database_url = &config.database.url;

    info!("DB 연결 시도...");
    let db_pool = DbPool::connect(database_url, &DbPoolConfig::default()).await?;
    db_pool.health_check().await?;
    info!("DB 연결 성공");

    let session_repo = SessionRepository::new(db_pool.inner().clone());
    let position_store = DbPositionStore::new(db_pool.inner().clone());

    // ---------------------------------------------------------------
    // 3. 세션 생성 + Crash Recovery
    // ---------------------------------------------------------------
    // ZScoreConfig는 Serialize를 구현하지 않으므로,
    // 주요 설정값을 JSON으로 수동 직렬화
    let config_json = serde_json::to_string(&serde_json::json!({
        "coins": strategy_config.coins,
        "window_size": strategy_config.window_size,
        "entry_z_threshold": strategy_config.entry_z_threshold,
        "exit_z_threshold": strategy_config.exit_z_threshold,
        "total_capital_usdt": strategy_config.total_capital_usdt.to_string(),
        "leverage": strategy_config.leverage,
        "auto_select": strategy_config.auto_select,
        "max_coins": strategy_config.max_coins,
        "kill_switch_enabled": strategy_config.kill_switch_enabled,
        "order_type": strategy_config.order_type,
    }))?;

    // 이전 Running 세션 확인 (crash recovery)
    let parent_session_id = match session_repo.find_last_running().await {
        Ok(Some(prev)) => {
            warn!(
                prev_session_id = prev.id,
                started_at = %prev.started_at,
                "이전 Running 세션 발견 — crash recovery 모드"
            );
            Some(prev.id)
        }
        Ok(None) => {
            info!("이전 Running 세션 없음 — clean start");
            None
        }
        Err(e) => {
            warn!(error = %e, "이전 세션 조회 실패, clean start로 진행");
            None
        }
    };

    let session_id = session_repo
        .create_session(&config_json, parent_session_id)
        .await?;
    info!(session_id = session_id, "새 세션 생성 완료");

    // 이전 세션을 Crashed로 마킹
    if let Some(prev_id) = parent_session_id
        && let Err(e) = session_repo.mark_crashed(prev_id, session_id).await
    {
        warn!(prev_session_id = prev_id, error = %e, "Crashed 마킹 실패");
    }

    // Crash recovery: 이전 세션의 미결 포지션 감지
    if let Some(prev_id) = parent_session_id {
        use arb_poc::db::positions::PositionStore as DbPositionStoreTrait;
        match position_store.load_open(prev_id).await {
            Ok(open_positions) if !open_positions.is_empty() => {
                warn!(
                    prev_session_id = prev_id,
                    open_count = open_positions.len(),
                    coins = ?open_positions.iter().map(|p| p.coin.as_str()).collect::<Vec<_>>(),
                    "이전 세션에 미결 포지션 발견 — 수동 확인 필요"
                );
                // TODO: LivePolicy 초기화 시 자동 복구/청산 로직 추가
                // 현재는 경고만 출력하고 새 세션으로 진행
            }
            Ok(_) => {
                info!(prev_session_id = prev_id, "이전 세션 미결 포지션 없음");
            }
            Err(e) => {
                warn!(
                    prev_session_id = prev_id,
                    error = %e,
                    "이전 세션 포지션 조회 실패"
                );
            }
        }
    }

    // ---------------------------------------------------------------
    // 4. 거래소 클라이언트 생성 (인증)
    // ---------------------------------------------------------------
    let upbit = UpbitClient::with_credentials(&config.upbit.api_key, &config.upbit.secret_key)?;
    let bybit = BybitClient::with_credentials(&config.bybit.api_key, &config.bybit.secret_key)?
        .with_category("linear");

    info!(
        upbit = upbit.name(),
        bybit = bybit.name(),
        "거래소 클라이언트 생성 완료 (인증)"
    );

    // ---------------------------------------------------------------
    // 4-1. Bybit leverage/margin mode 검증
    // ---------------------------------------------------------------
    // auto_select가 아닌 경우, 설정된 코인 목록에 대해 레버리지/마진 모드 설정
    if !strategy_config.auto_select {
        let leverage = strategy_config.leverage;
        for coin in &strategy_config.coins {
            let symbol = format!("{coin}USDT");
            // Cross margin 모드 설정 (trade_mode=0)
            if let Err(e) = bybit.switch_margin_mode(&symbol, 0, leverage).await {
                warn!(
                    symbol = %symbol,
                    error = %e,
                    "Bybit 마진 모드 전환 실패 (non-fatal)"
                );
            }
            // 레버리지 설정
            if let Err(e) = bybit.set_leverage(&symbol, leverage).await {
                warn!(
                    symbol = %symbol,
                    leverage = leverage,
                    error = %e,
                    "Bybit 레버리지 설정 실패 (non-fatal)"
                );
            }
        }
        info!(
            leverage = leverage,
            coins_count = strategy_config.coins.len(),
            "Bybit 레버리지/마진 모드 검증 완료"
        );
    } else {
        info!("auto_select 모드 — Bybit 레버리지는 코인 선택 시 동적 설정");
    }

    // ---------------------------------------------------------------
    // 5. 실잔고 조회 → BalanceTracker 초기화
    // ---------------------------------------------------------------
    // Balance 구조체: { currency, balance, locked, avg_buy_price, unit_currency }
    // balance 필드가 가용 잔고
    let upbit_krw_balance = match upbit.get_balance("KRW").await {
        Ok(bal) => {
            info!(balance = %bal.balance, locked = %bal.locked, "Upbit KRW 잔고 조회");
            bal.balance
        }
        Err(e) => {
            return Err(format!("Upbit 잔고 조회 실패: {e}").into());
        }
    };

    let bybit_usdt_balance = match bybit.get_balance("USDT").await {
        Ok(bal) => {
            info!(balance = %bal.balance, locked = %bal.locked, "Bybit USDT 잔고 조회");
            bal.balance
        }
        Err(e) => {
            return Err(format!("Bybit 잔고 조회 실패: {e}").into());
        }
    };

    let balance_tracker = Arc::new(BalanceTracker::new(upbit_krw_balance, bybit_usdt_balance));

    info!(
        upbit_krw = %upbit_krw_balance,
        bybit_usdt = %bybit_usdt_balance,
        "BalanceTracker 초기화 완료"
    );

    // ---------------------------------------------------------------
    // 6. RiskManager 초기화
    // ---------------------------------------------------------------
    let risk_config = RiskConfig {
        max_daily_loss_pct: strategy_config.max_daily_loss_pct,
        max_daily_loss_usdt: Decimal::try_from(strategy_config.max_daily_loss_usdt)
            .unwrap_or(Decimal::from(50)),
        max_drawdown_pct: strategy_config.max_drawdown_pct,
        max_drawdown_usdt: Decimal::try_from(strategy_config.max_drawdown_usdt)
            .unwrap_or(Decimal::from(25)),
        max_single_loss_pct: strategy_config.max_single_loss_pct,
        max_single_loss_usdt: Decimal::try_from(strategy_config.max_single_loss_usdt)
            .unwrap_or(Decimal::from(15)),
        max_order_size_usdt: Decimal::try_from(strategy_config.max_order_size_usdt)
            .unwrap_or(Decimal::from(2000)),
        max_concurrent_positions: strategy_config
            .max_concurrent_positions
            .unwrap_or(strategy_config.coins.len()),
        max_rolling_24h_loss_usdt: Decimal::try_from(strategy_config.max_rolling_24h_loss_usdt)
            .unwrap_or(Decimal::from(80)),
        total_capital_usdt: strategy_config.total_capital_usdt,
        kill_switch_enabled: strategy_config.kill_switch_enabled,
        ..RiskConfig::default()
    };

    let risk_manager = Arc::new(RiskManager::new(risk_config));
    info!("RiskManager 초기화 완료");

    // ---------------------------------------------------------------
    // 6-1. AlertService 생성
    // ---------------------------------------------------------------
    let alert_consumer = if config.telegram.is_configured() {
        info!("텔레그램 알림 활성화");
        let telegram_client = Arc::new(
            arb_poc::telegram::TelegramClient::new(&config.telegram)
                .map_err(|e| format!("Telegram 클라이언트 생성 실패: {e}"))?,
        );

        let alert_repo = arb_poc::db::alerts::AlertRepository::new(db_pool.inner().clone());

        // 텔레그램 전송 클로저
        let tg = Arc::clone(&telegram_client);
        let telegram_send_fn = move |msg: String| -> std::pin::Pin<
            Box<dyn std::future::Future<Output = Result<(), String>> + Send>,
        > {
            let tg = Arc::clone(&tg);
            Box::pin(async move {
                tg.send_message(&msg).await.map_err(|e| e.to_string())?;
                Ok(())
            })
        };

        // DB fallback 클로저
        let db_alert_fn = move |sid: i64,
                                level: &str,
                                event_type: &str,
                                message: &str,
                                payload: Option<String>|
              -> std::pin::Pin<
            Box<dyn std::future::Future<Output = Result<(), String>> + Send>,
        > {
            let repo = alert_repo.clone();
            let record = arb_poc::db::alerts::AlertRecord {
                id: None,
                session_id: sid,
                level: level.to_string(),
                event_type: event_type.to_string(),
                message: message.to_string(),
                payload_json: payload,
                created_at: chrono::Utc::now(),
            };
            Box::pin(async move {
                repo.insert_alert(&record)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(())
            })
        };

        let (_alert_service, consumer) = arb_poc::strategy::zscore::alert::AlertService::new(
            session_id,
            telegram_send_fn,
            db_alert_fn,
        );
        info!("AlertService 생성 완료 (텔레그램 + DB fallback)");
        Some(consumer)
    } else {
        info!("텔레그램 미설정 — AlertService 비활성화");
        None
    };

    // ---------------------------------------------------------------
    // 7. DbWriter + BalanceRecorderTask 생성
    // ---------------------------------------------------------------
    let strategy_config_arc = Arc::new(strategy_config);

    // DbWriter 생성 (background DB write 파이프라인)
    let db_writer_session_repo = SessionRepository::new(db_pool.inner().clone());
    let db_writer_position_store = DbPositionStore::new(db_pool.inner().clone());
    let db_writer_trade_repo = TradeRepository::new(db_pool.inner().clone());
    let db_writer_minute_repo = MinuteRepository::new(db_pool.inner().clone());
    let db_writer_alert_repo = AlertRepository::new(db_pool.inner().clone());
    let db_writer_funding_repo = FundingRepository::new(db_pool.inner().clone());
    let db_writer_balance_repo = BalanceSnapshotRepository::new(db_pool.inner().clone());

    let db_writer = DbWriter::new(
        db_writer_session_repo,
        db_writer_position_store,
        db_writer_trade_repo,
        db_writer_minute_repo,
        db_writer_alert_repo,
        db_writer_funding_repo,
        db_writer_balance_repo,
    );
    info!("DbWriter 생성 완료");

    // ForexCache 생성 (USD/KRW 공시 환율)
    let forex_cache = Arc::new(ForexCache::new(Duration::from_secs(600)));

    // UsdtKrwCache 생성 (USDT/KRW 거래소 시세)
    let usdt_krw_cache = Arc::new(UsdtKrwCache::new());

    // Upbit REST로 USDT/KRW 초기값 조회
    match upbit.get_ticker(&["KRW-USDT"]).await {
        Ok(tickers) if !tickers.is_empty() => {
            let price = tickers[0].trade_price;
            if let Some(price_f64) = price.to_f64() {
                usdt_krw_cache.update(price_f64);
                info!(usdt_krw = price_f64, "USDT/KRW 초기값 설정");
            } else {
                warn!(price = %price, "USDT/KRW Decimal->f64 변환 실패");
            }
        }
        Ok(_) => warn!("USDT/KRW ticker 응답 비어있음"),
        Err(e) => warn!(error = %e, "USDT/KRW 초기값 조회 실패"),
    }

    // ExchangeAdapter 생성 (잔고 조회용)
    let upbit_adapter: Arc<dyn ExchangeAdapter> = Arc::new(UpbitAdapter::new(upbit.clone()));
    let bybit_adapter: Arc<dyn ExchangeAdapter> = Arc::new(BybitAdapter::new(bybit.clone()));

    // BalanceRecorderTask 시작
    let snapshot_interval = strategy_config_arc.balance_snapshot.interval_sec;
    let (snapshot_sender, recorder_task) = BalanceRecorderTask::spawn(
        session_id,
        upbit_adapter,
        bybit_adapter,
        Arc::clone(&forex_cache),
        Arc::clone(&usdt_krw_cache),
        db_writer.clone(),
        snapshot_interval,
    );
    info!(interval_sec = snapshot_interval, "BalanceRecorderTask 시작");

    // ---------------------------------------------------------------
    // 8. LiveExecutor + LivePolicy 생성
    // ---------------------------------------------------------------
    // 클라이언트를 clone하여 LiveExecutor용과 ZScoreMonitor용으로 분리.
    // Clone 구현은 Arc 기반이므로 커넥션 풀과 rate limiter를 공유합니다.
    let upbit_for_executor = upbit.clone();
    let bybit_for_executor = bybit.clone();

    let executor = Arc::new(LiveExecutor::new(
        Arc::new(upbit_for_executor),
        Arc::new(bybit_for_executor),
        Arc::clone(&strategy_config_arc),
    ));

    let adapter = DbPositionStoreAdapter::new(position_store);
    let position_store_arc = Arc::new(adapter);

    let policy = LivePolicy::new(
        executor,
        Arc::clone(&balance_tracker),
        Arc::clone(&risk_manager),
        Arc::clone(&position_store_arc),
        session_id,
        Some(snapshot_sender.clone()),
    );

    info!(session_id = session_id, "LivePolicy 생성 완료");

    // ---------------------------------------------------------------
    // 9. ZScoreMonitor 생성
    // ---------------------------------------------------------------
    let config_for_monitor = (*strategy_config_arc).clone();

    let monitor = ZScoreMonitor::new(upbit, bybit, config_for_monitor, forex_cache, policy);

    // ---------------------------------------------------------------
    // 10. Graceful Shutdown 핸들러
    // ---------------------------------------------------------------
    let cancel_token = CancellationToken::new();

    let cancel_clone = cancel_token.clone();
    tokio::spawn(async move {
        // SIGINT (Ctrl+C)
        tokio::signal::ctrl_c().await.ok();
        info!("SIGINT 수신 — graceful shutdown 시작...");
        cancel_clone.cancel();
    });

    // SIGTERM 핸들러 (Unix only)
    #[cfg(unix)]
    {
        let cancel_term = cancel_token.clone();
        tokio::spawn(async move {
            let mut sigterm =
                tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                    .expect("SIGTERM 핸들러 등록 실패");
            sigterm.recv().await;
            info!("SIGTERM 수신 — graceful shutdown 시작...");
            cancel_term.cancel();
        });
    }

    // ---------------------------------------------------------------
    // 11. 세션 heartbeat task
    // ---------------------------------------------------------------
    let session_repo_hb = SessionRepository::new(db_pool.inner().clone());
    let session_id_hb = session_id;
    let cancel_hb = cancel_token.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        loop {
            tokio::select! {
                _ = cancel_hb.cancelled() => break,
                _ = interval.tick() => {
                    if let Err(e) = session_repo_hb.update_heartbeat(session_id_hb).await {
                        warn!(error = %e, "heartbeat 갱신 실패");
                    }
                }
            }
        }
    });

    // ---------------------------------------------------------------
    // 12. 모니터링 실행
    // ---------------------------------------------------------------
    info!("=== 실시간 모니터링 시작 (라이브 모드) ===");

    let trades: Vec<ClosedPosition> = match monitor.run(cancel_token).await {
        Ok(t) => t,
        Err(e) => {
            error!(error = %e, "모니터링 실행 실패");
            // 세션 상태 Errored로 업데이트
            let _ = session_repo.end_session(session_id, "Errored").await;
            let err: Box<dyn std::error::Error> = Box::new(e);
            return Err(err);
        }
    };

    // ---------------------------------------------------------------
    // 13. BalanceRecorderTask 종료
    // ---------------------------------------------------------------
    info!("BalanceRecorderTask 종료 요청...");
    snapshot_sender.shutdown().await;
    match tokio::time::timeout(Duration::from_secs(60), recorder_task).await {
        Ok(Ok(())) => info!("BalanceRecorderTask 정상 종료"),
        Ok(Err(e)) => warn!(error = %e, "BalanceRecorderTask 종료 에러"),
        Err(_) => warn!("BalanceRecorderTask 종료 타임아웃 (60초), task 포기"),
    }

    // ---------------------------------------------------------------
    // 14. 세션 종료
    // ---------------------------------------------------------------
    let session_status = if risk_manager.is_killed() {
        "KillSwitched"
    } else {
        "GracefulStop"
    };

    if let Err(e) = session_repo.end_session(session_id, session_status).await {
        warn!(error = %e, "세션 종료 UPDATE 실패");
    }

    // 결과 출력
    info!("=== 모니터링 결과 ===");
    info!(total_trades = trades.len(), "총 거래 수");

    if !trades.is_empty() {
        let winning = trades.iter().filter(|t| t.net_pnl > Decimal::ZERO).count();
        let net_pnl: Decimal = trades.iter().map(|t| t.net_pnl).sum();

        info!(
            winning = winning,
            losing = trades.len() - winning,
            net_pnl = %net_pnl,
            "거래 결과"
        );
    }

    // AlertService consumer 종료 대기
    if let Some(consumer) = alert_consumer {
        info!("AlertService consumer 종료 대기...");
        consumer.shutdown().await;
    }

    info!(
        session_id = session_id,
        status = session_status,
        "=== 라이브 트레이딩 종료 ==="
    );

    Ok(())
}
