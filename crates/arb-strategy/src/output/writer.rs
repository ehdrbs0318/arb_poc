//! 세션 파일 출력 (CSV/JSON 이중 저장).
//!
//! `SessionWriter`는 세션 디렉토리를 관리하며,
//! 거래 내역과 분봉 통계를 CSV 실시간 append + JSON 종료 시 일괄 저장합니다.

use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use super::summary::SessionSummary;
use crate::zscore::pnl::ClosedPosition;

/// 출력 설정.
#[derive(Debug, Clone, Deserialize)]
pub struct OutputConfig {
    /// 파일 출력 활성화 여부 (기본값: true).
    pub enabled: bool,
    /// 출력 기본 디렉토리 (기본값: "output").
    pub dir: String,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            dir: "output".to_string(),
        }
    }
}

/// 분봉 통계 1행 기록.
///
/// 매 분 완결 시 `minutes.csv`에 append되는 단일 레코드입니다.
#[derive(Debug, Clone, Serialize)]
pub struct MinuteRecord {
    /// 분봉 시각 (UTC).
    pub timestamp: String,
    /// 코인 심볼.
    pub coin: String,
    /// Upbit 종가 (USD 환산).
    pub upbit_close: f64,
    /// Bybit 종가 (USDT).
    pub bybit_close: f64,
    /// 해당 분의 USD/KRW 환율.
    pub usd_krw: f64,
    /// 스프레드 (%).
    pub spread_pct: f64,
    /// Rolling mean.
    pub mean: f64,
    /// Rolling stddev.
    pub stddev: f64,
    /// Z-Score.
    pub z_score: f64,
    /// 포지션 상태 ("OPEN" / "NONE").
    pub position: String,
    /// 데이터 출처 ("warmup" / "live").
    pub source: String,
}

/// 세션 파일 writer.
///
/// 세션 디렉토리를 생성하고 CSV/JSON 파일에 거래 내역과 분봉 통계를 기록합니다.
/// CSV는 실시간 append + flush, JSON은 종료 시 일괄 저장합니다.
pub struct SessionWriter {
    /// 세션 디렉토리 경로.
    session_dir: PathBuf,
    /// trades.csv BufWriter.
    trades_writer: BufWriter<File>,
    /// minutes.csv BufWriter.
    minutes_writer: BufWriter<File>,
    /// trades.csv 헤더 기록 여부.
    trades_header_written: bool,
    /// minutes.csv 헤더 기록 여부.
    minutes_header_written: bool,
}

impl SessionWriter {
    /// 새 `SessionWriter`를 생성합니다.
    ///
    /// `enabled=false`이면 `None`을 반환합니다.
    /// 세션 디렉토리를 자동 생성하고 CSV 파일을 초기화합니다.
    ///
    /// # 인자
    ///
    /// * `config` - 출력 설정
    ///
    /// # 반환값
    ///
    /// `enabled=true`이면 `Some(SessionWriter)`, 아니면 `None`.
    pub fn new(config: &OutputConfig) -> io::Result<Option<Self>> {
        if !config.enabled {
            debug!("파일 출력 비활성화");
            return Ok(None);
        }

        // 타임스탬프 형식: YYYY-MM-DD_HH-mm-ss
        let now = Utc::now();
        let timestamp = now.format("%Y-%m-%d_%H-%M-%S").to_string();
        let session_dir = PathBuf::from(&config.dir).join(&timestamp);

        // 세션 디렉토리 생성
        fs::create_dir_all(&session_dir)?;
        info!(dir = %session_dir.display(), "세션 출력 디렉토리 생성");

        // CSV 파일 생성
        let trades_file = open_csv_file(&session_dir, "trades.csv")?;
        let minutes_file = open_csv_file(&session_dir, "minutes.csv")?;

        Ok(Some(Self {
            session_dir,
            trades_writer: BufWriter::new(trades_file),
            minutes_writer: BufWriter::new(minutes_file),
            trades_header_written: false,
            minutes_header_written: false,
        }))
    }

    /// 지정된 디렉토리에 `SessionWriter`를 생성합니다 (테스트용).
    ///
    /// # 인자
    ///
    /// * `session_dir` - 세션 디렉토리 경로
    pub fn with_dir(session_dir: PathBuf) -> io::Result<Self> {
        fs::create_dir_all(&session_dir)?;

        let trades_file = open_csv_file(&session_dir, "trades.csv")?;
        let minutes_file = open_csv_file(&session_dir, "minutes.csv")?;

        Ok(Self {
            session_dir,
            trades_writer: BufWriter::new(trades_file),
            minutes_writer: BufWriter::new(minutes_file),
            trades_header_written: false,
            minutes_header_written: false,
        })
    }

    /// 세션 디렉토리 경로를 반환합니다.
    pub fn session_dir(&self) -> &Path {
        &self.session_dir
    }

    /// 거래 내역 1건을 `trades.csv`에 append합니다.
    ///
    /// 첫 호출 시 CSV 헤더를 기록합니다. 매 append 후 flush합니다.
    pub fn append_trade(&mut self, trade: &ClosedPosition) -> io::Result<()> {
        // 헤더 기록 (최초 1회)
        if !self.trades_header_written {
            writeln!(
                self.trades_writer,
                "coin,entry_time,exit_time,holding_minutes,size_usdt,\
                 upbit_entry_price,bybit_entry_price,upbit_exit_price,bybit_exit_price,\
                 entry_spread_pct,exit_spread_pct,entry_z_score,exit_z_score,\
                 entry_usd_krw,exit_usd_krw,upbit_pnl,bybit_pnl,\
                 upbit_fees,bybit_fees,total_fees,net_pnl,is_liquidated"
            )?;
            self.trades_header_written = true;
        }

        writeln!(
            self.trades_writer,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            trade.coin,
            trade.entry_time.to_rfc3339(),
            trade.exit_time.to_rfc3339(),
            trade.holding_minutes,
            trade.size_usdt,
            trade.upbit_entry_price,
            trade.bybit_entry_price,
            trade.upbit_exit_price,
            trade.bybit_exit_price,
            trade.entry_spread_pct,
            trade.exit_spread_pct,
            trade.entry_z_score,
            trade.exit_z_score,
            trade.entry_usd_krw,
            trade.exit_usd_krw,
            trade.upbit_pnl,
            trade.bybit_pnl,
            trade.upbit_fees,
            trade.bybit_fees,
            trade.total_fees,
            trade.net_pnl,
            trade.is_liquidated
        )?;

        self.trades_writer.flush()?;
        debug!(coin = %trade.coin, net_pnl = %trade.net_pnl, "거래 내역 CSV append");
        Ok(())
    }

    /// 분봉 통계 1건을 `minutes.csv`에 append합니다.
    ///
    /// 첫 호출 시 CSV 헤더를 기록합니다. 매 append 후 flush합니다.
    pub fn append_minute(&mut self, record: &MinuteRecord) -> io::Result<()> {
        self.ensure_minutes_header()?;

        write_minute_row(&mut self.minutes_writer, record)?;
        self.minutes_writer.flush()?;

        Ok(())
    }

    /// 워밍업 데이터를 일괄 기록합니다.
    ///
    /// 헤더 기록 후 모든 레코드를 append하고 flush합니다.
    pub fn append_minutes_batch(&mut self, records: &[MinuteRecord]) -> io::Result<()> {
        if records.is_empty() {
            return Ok(());
        }

        self.ensure_minutes_header()?;

        for record in records {
            write_minute_row(&mut self.minutes_writer, record)?;
        }
        self.minutes_writer.flush()?;

        debug!(count = records.len(), "워밍업 분봉 데이터 일괄 기록");
        Ok(())
    }

    /// 세션 종료 시 JSON 파일을 일괄 저장합니다.
    ///
    /// `trades.json`, `minutes.json`, `summary.json`, `summary.txt`를 생성합니다.
    pub fn finalize(
        &mut self,
        trades: &[ClosedPosition],
        minutes: &[MinuteRecord],
        summary: &SessionSummary,
    ) -> io::Result<()> {
        // trades.json
        let trades_json_path = self.session_dir.join("trades.json");
        let trades_json = serde_json::to_string_pretty(trades).map_err(io::Error::other)?;
        fs::write(&trades_json_path, trades_json)?;
        debug!(path = %trades_json_path.display(), "trades.json 저장");

        // minutes.json
        let minutes_json_path = self.session_dir.join("minutes.json");
        let minutes_json = serde_json::to_string_pretty(minutes).map_err(io::Error::other)?;
        fs::write(&minutes_json_path, minutes_json)?;
        debug!(path = %minutes_json_path.display(), "minutes.json 저장");

        // summary.json
        let summary_json_path = self.session_dir.join("summary.json");
        let summary_json = serde_json::to_string_pretty(summary).map_err(io::Error::other)?;
        fs::write(&summary_json_path, summary_json)?;
        debug!(path = %summary_json_path.display(), "summary.json 저장");

        // summary.txt
        let summary_txt_path = self.session_dir.join("summary.txt");
        fs::write(&summary_txt_path, summary.to_text())?;
        debug!(path = %summary_txt_path.display(), "summary.txt 저장");

        info!(dir = %self.session_dir.display(), "세션 파일 저장 완료");
        Ok(())
    }

    /// minutes.csv 헤더를 기록합니다 (최초 1회).
    fn ensure_minutes_header(&mut self) -> io::Result<()> {
        if !self.minutes_header_written {
            writeln!(
                self.minutes_writer,
                "timestamp,coin,upbit_close,bybit_close,usd_krw,\
                 spread_pct,mean,stddev,z_score,position,source"
            )?;
            self.minutes_header_written = true;
        }
        Ok(())
    }
}

/// 분봉 통계 1행을 writer에 기록합니다 (flush 없음).
fn write_minute_row<W: Write>(writer: &mut W, record: &MinuteRecord) -> io::Result<()> {
    writeln!(
        writer,
        "{},{},{},{},{},{},{},{},{},{},{}",
        record.timestamp,
        record.coin,
        record.upbit_close,
        record.bybit_close,
        record.usd_krw,
        record.spread_pct,
        record.mean,
        record.stddev,
        record.z_score,
        record.position,
        record.source
    )
}

/// CSV 파일을 생성/오픈합니다 (append 모드).
fn open_csv_file(dir: &Path, filename: &str) -> io::Result<File> {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join(filename))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use rust_decimal::Decimal;

    /// 테스트용 `ClosedPosition` 생성 헬퍼.
    fn make_trade(coin: &str, net_pnl: Decimal) -> ClosedPosition {
        let now = Utc::now();
        ClosedPosition {
            coin: coin.to_string(),
            entry_time: now,
            exit_time: now,
            holding_minutes: 30,
            size_usdt: Decimal::new(1000, 0),
            upbit_entry_price: Decimal::new(95_000, 0),
            bybit_entry_price: Decimal::new(95_100, 0),
            upbit_exit_price: Decimal::new(95_050, 0),
            bybit_exit_price: Decimal::new(95_080, 0),
            upbit_pnl: Decimal::ZERO,
            bybit_pnl: Decimal::ZERO,
            upbit_fees: Decimal::ZERO,
            bybit_fees: Decimal::ZERO,
            total_fees: Decimal::ZERO,
            net_pnl,
            entry_z_score: 2.0,
            exit_z_score: 0.5,
            entry_spread_pct: 0.3,
            exit_spread_pct: 0.1,
            entry_usd_krw: 1380.0,
            exit_usd_krw: 1381.0,
            is_liquidated: false,
        }
    }

    /// 테스트용 `MinuteRecord` 생성 헬퍼.
    fn make_minute(coin: &str, z_score: f64, source: &str) -> MinuteRecord {
        MinuteRecord {
            timestamp: Utc::now().to_rfc3339(),
            coin: coin.to_string(),
            upbit_close: 95000.0,
            bybit_close: 95100.0,
            usd_krw: 1380.0,
            spread_pct: 0.105,
            mean: 0.1,
            stddev: 0.05,
            z_score,
            position: "NONE".to_string(),
            source: source.to_string(),
        }
    }

    #[test]
    fn test_session_writer_disabled() {
        let config = OutputConfig {
            enabled: false,
            dir: "output".to_string(),
        };
        let writer = SessionWriter::new(&config).unwrap();
        assert!(writer.is_none());
    }

    #[test]
    fn test_session_writer_creates_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let session_dir = tmp.path().join("test_session");

        let _writer = SessionWriter::with_dir(session_dir.clone()).unwrap();

        assert!(session_dir.exists());
        assert!(session_dir.join("trades.csv").exists());
        assert!(session_dir.join("minutes.csv").exists());
    }

    #[test]
    fn test_append_trade_csv() {
        let tmp = tempfile::tempdir().unwrap();
        let session_dir = tmp.path().join("test_trades");

        let mut writer = SessionWriter::with_dir(session_dir.clone()).unwrap();

        let trade = make_trade("BTC", Decimal::new(10, 0));
        writer.append_trade(&trade).unwrap();

        // 두 번째 거래 추가
        let trade2 = make_trade("ETH", Decimal::new(-5, 0));
        writer.append_trade(&trade2).unwrap();

        // CSV 파일 읽기
        let content = fs::read_to_string(session_dir.join("trades.csv")).unwrap();
        let lines: Vec<&str> = content.lines().collect();

        // 헤더 + 데이터 2행 = 3행
        assert_eq!(lines.len(), 3);
        assert!(lines[0].starts_with("coin,"));
        assert!(lines[1].starts_with("BTC,"));
        assert!(lines[2].starts_with("ETH,"));
    }

    #[test]
    fn test_append_minute_csv() {
        let tmp = tempfile::tempdir().unwrap();
        let session_dir = tmp.path().join("test_minutes");

        let mut writer = SessionWriter::with_dir(session_dir.clone()).unwrap();

        let record = make_minute("BTC", 1.5, "live");
        writer.append_minute(&record).unwrap();

        let record2 = make_minute("XRP", 2.3, "warmup");
        writer.append_minute(&record2).unwrap();

        // CSV 파일 읽기
        let content = fs::read_to_string(session_dir.join("minutes.csv")).unwrap();
        let lines: Vec<&str> = content.lines().collect();

        // 헤더 + 데이터 2행 = 3행
        assert_eq!(lines.len(), 3);
        assert!(lines[0].starts_with("timestamp,"));
        assert!(lines[1].contains("BTC"));
        assert!(lines[2].contains("XRP"));
    }

    #[test]
    fn test_append_minutes_batch() {
        let tmp = tempfile::tempdir().unwrap();
        let session_dir = tmp.path().join("test_batch");

        let mut writer = SessionWriter::with_dir(session_dir.clone()).unwrap();

        let records = vec![
            make_minute("BTC", 1.0, "warmup"),
            make_minute("BTC", 1.5, "warmup"),
            make_minute("BTC", 2.0, "warmup"),
        ];

        writer.append_minutes_batch(&records).unwrap();

        let content = fs::read_to_string(session_dir.join("minutes.csv")).unwrap();
        let lines: Vec<&str> = content.lines().collect();

        // 헤더 + 데이터 3행 = 4행
        assert_eq!(lines.len(), 4);
        assert!(lines[0].starts_with("timestamp,"));
    }

    #[test]
    fn test_finalize_creates_json() {
        let tmp = tempfile::tempdir().unwrap();
        let session_dir = tmp.path().join("test_finalize");

        let mut writer = SessionWriter::with_dir(session_dir.clone()).unwrap();

        let start = Utc.with_ymd_and_hms(2026, 2, 9, 20, 30, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2026, 2, 10, 8, 30, 0).unwrap();
        let coins = vec!["BTC".to_string()];

        let trades = vec![make_trade("BTC", Decimal::new(10, 0))];
        let minutes = vec![make_minute("BTC", 1.5, "live")];
        let summary =
            SessionSummary::calculate(&trades, start, end, &coins, 1463.33, 1465.20, 54320);

        writer.finalize(&trades, &minutes, &summary).unwrap();

        // JSON 파일 존재 확인
        assert!(session_dir.join("trades.json").exists());
        assert!(session_dir.join("minutes.json").exists());
        assert!(session_dir.join("summary.json").exists());
        assert!(session_dir.join("summary.txt").exists());

        // summary.json 내용 확인
        let summary_json = fs::read_to_string(session_dir.join("summary.json")).unwrap();
        assert!(summary_json.contains("total_trades"));
        assert!(summary_json.contains("win_rate"));

        // summary.txt 내용 확인
        let summary_txt = fs::read_to_string(session_dir.join("summary.txt")).unwrap();
        assert!(summary_txt.contains("세션 요약"));
    }

    #[test]
    fn test_output_config_default() {
        let config = OutputConfig::default();
        assert!(config.enabled);
        assert_eq!(config.dir, "output");
    }

    #[test]
    fn test_empty_batch_is_noop() {
        let tmp = tempfile::tempdir().unwrap();
        let session_dir = tmp.path().join("test_empty_batch");

        let mut writer = SessionWriter::with_dir(session_dir.clone()).unwrap();

        // 빈 배열 append
        writer.append_minutes_batch(&[]).unwrap();

        // minutes.csv는 빈 파일 (헤더도 없음)
        let content = fs::read_to_string(session_dir.join("minutes.csv")).unwrap();
        assert!(content.is_empty());
    }
}
