//! 볼륨/변동성 기반 자동 코인 선택 모듈.
//!
//! 양쪽 거래소의 전종목 티커를 조회하여 교집합을 구하고,
//! 거래량과 변동성 기준으로 최적의 코인을 자동 선택합니다.

use crate::error::StrategyError;
use arb_exchange::{CandleInterval, MarketData, Ticker};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use std::collections::HashSet;
use tracing::{debug, info, warn};

/// 스테이블코인 목록 (자동 선택에서 제외).
const STABLECOINS: &[&str] = &[
    "USDT", "USDC", "DAI", "TUSD", "BUSD", "FDUSD", "PYUSD", "UST",
];

/// 코인 후보 정보.
#[derive(Debug, Clone)]
pub struct CoinCandidate {
    /// 코인 심볼 (예: "BTC", "ETH").
    pub coin: String,
    /// 1시간 거래량 (USDT 환산).
    pub volume_1h_usdt: f64,
    /// 24시간 변동성 ((high - low) / low × 100).
    pub volatility_24h_pct: f64,
}

/// 볼륨/변동성 기반 자동 코인 선택기.
///
/// 양쪽 거래소(Upbit, Bybit)의 시장 데이터를 활용하여
/// 차익거래에 적합한 코인을 자동으로 선택합니다.
pub struct CoinSelector<'a, U: MarketData, B: MarketData> {
    upbit: &'a U,
    bybit: &'a B,
}

impl<'a, U: MarketData, B: MarketData> CoinSelector<'a, U, B> {
    /// 새 CoinSelector를 생성합니다.
    ///
    /// # 인자
    ///
    /// * `upbit` - Upbit MarketData 구현체
    /// * `bybit` - Bybit MarketData 구현체
    pub fn new(upbit: &'a U, bybit: &'a B) -> Self {
        Self { upbit, bybit }
    }

    /// 볼륨/변동성 기반으로 최적의 코인 목록을 선택합니다.
    ///
    /// # 알고리즘
    ///
    /// 1. 양쪽 거래소 전종목 Ticker 조회
    /// 2. 교집합 추출 (공통 코인)
    /// 3. 스테이블코인/블랙리스트 제외
    /// 4. 24h 거래대금 하위 50% 제거
    /// 5. 1h 캔들로 거래량 필터링
    /// 6. 변동성 내림차순 정렬 → 상위 N개 반환
    ///
    /// # 인자
    ///
    /// * `max_coins` - 최대 반환 코인 수
    /// * `min_volume_1h_usdt` - 1시간 최소 거래량 (USDT)
    /// * `blacklist` - 제외할 코인 심볼 목록
    pub async fn select(
        &self,
        max_coins: usize,
        min_volume_1h_usdt: Decimal,
        blacklist: &[String],
    ) -> Result<Vec<CoinCandidate>, StrategyError> {
        // 1. 양쪽 거래소 전종목 Ticker 조회
        let upbit_tickers = self.upbit.get_all_tickers().await?;
        let bybit_tickers = self.bybit.get_all_tickers().await?;

        info!(
            upbit_count = upbit_tickers.len(),
            bybit_count = bybit_tickers.len(),
            "전종목 티커 조회 완료"
        );

        // 2. 교집합 추출: 코인 심볼 기준
        let upbit_coins = extract_coins_upbit(&upbit_tickers);
        let bybit_coins = extract_coins_bybit(&bybit_tickers);
        let common_coins: HashSet<&str> = upbit_coins.intersection(&bybit_coins).copied().collect();

        debug!(common_count = common_coins.len(), "교집합 코인 수");

        // 3. 스테이블코인 제외
        let stablecoin_set: HashSet<&str> = STABLECOINS.iter().copied().collect();
        let blacklist_set: HashSet<String> = blacklist.iter().map(|s| s.to_uppercase()).collect();

        let filtered_coins: Vec<&str> = common_coins
            .into_iter()
            .filter(|coin| !stablecoin_set.contains(*coin))
            .filter(|coin| !blacklist_set.contains(*coin))
            .collect();

        debug!(
            after_filter = filtered_coins.len(),
            "스테이블코인/블랙리스트 필터 후"
        );

        if filtered_coins.is_empty() {
            info!("교집합에 유효한 코인이 없습니다");
            return Ok(Vec::new());
        }

        // 티커 맵 구축 (코인 → (upbit_ticker, bybit_ticker))
        let upbit_map = build_ticker_map_upbit(&upbit_tickers);
        let bybit_map = build_ticker_map_bybit(&bybit_tickers);

        // 5. 24h 거래대금 1차 필터: 양쪽 중 낮은 값 기준으로 하위 50% 제거
        let mut volume_pairs: Vec<(&str, Decimal)> = filtered_coins
            .iter()
            .filter_map(|coin| {
                let upbit_vol = upbit_map.get(*coin).map(|t| t.acc_trade_price_24h)?;
                let bybit_vol = bybit_map.get(*coin).map(|t| t.acc_trade_price_24h)?;
                let min_vol = upbit_vol.min(bybit_vol);
                Some((*coin, min_vol))
            })
            .collect();

        // 거래대금 오름차순 정렬 후 상위 50%만 유지
        volume_pairs.sort_by(|a, b| a.1.cmp(&b.1));
        let cutoff = volume_pairs.len() / 2;
        let top_half: Vec<&str> = volume_pairs[cutoff..]
            .iter()
            .map(|(coin, _)| *coin)
            .collect();

        debug!(
            before = volume_pairs.len(),
            after = top_half.len(),
            "24h 거래대금 상위 50% 필터"
        );

        // 6. 1h 캔들 조회 → 볼륨/변동성 계산
        let min_vol_f64 = min_volume_1h_usdt.to_f64().unwrap_or(0.0);
        let mut candidates: Vec<CoinCandidate> = Vec::new();

        for coin in &top_half {
            let upbit_market = U::market_code(coin, "KRW");
            let bybit_market = B::market_code(coin, "USDT");

            // Upbit 1h 캔들 조회
            let upbit_candle = match self
                .upbit
                .get_candles(&upbit_market, CandleInterval::Minute60, 1)
                .await
            {
                Ok(candles) if !candles.is_empty() => candles[0].clone(),
                Ok(_) => {
                    warn!(
                        coin = *coin,
                        exchange = "upbit",
                        "1h 캔들 데이터 없음, 건너뜀"
                    );
                    continue;
                }
                Err(e) => {
                    warn!(coin = *coin, exchange = "upbit", error = %e, "1h 캔들 조회 실패, 건너뜀");
                    continue;
                }
            };

            // Bybit 1h 캔들 조회
            let bybit_candle = match self
                .bybit
                .get_candles(&bybit_market, CandleInterval::Minute60, 1)
                .await
            {
                Ok(candles) if !candles.is_empty() => candles[0].clone(),
                Ok(_) => {
                    warn!(
                        coin = *coin,
                        exchange = "bybit",
                        "1h 캔들 데이터 없음, 건너뜀"
                    );
                    continue;
                }
                Err(e) => {
                    warn!(coin = *coin, exchange = "bybit", error = %e, "1h 캔들 조회 실패, 건너뜀");
                    continue;
                }
            };

            // Rate limit 보호: 100ms 딜레이
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;

            // 볼륨 USDT 환산: volume(코인수량) × close_price
            let upbit_vol_usdt = upbit_candle.volume.to_f64().unwrap_or(0.0)
                * upbit_candle.close.to_f64().unwrap_or(0.0);
            let bybit_vol_usdt = bybit_candle.volume.to_f64().unwrap_or(0.0)
                * bybit_candle.close.to_f64().unwrap_or(0.0);
            let volume_1h_usdt = upbit_vol_usdt.min(bybit_vol_usdt);

            // 7. 볼륨 임계값 필터
            if volume_1h_usdt < min_vol_f64 {
                debug!(
                    coin = *coin,
                    volume_1h_usdt, min_vol_f64, "1h 거래량 미달, 건너뜀"
                );
                continue;
            }

            // 8. 변동성 계산: Bybit Ticker 기준 (high - low) / low × 100
            let volatility_24h_pct = if let Some(bybit_ticker) = bybit_map.get(*coin) {
                let high = bybit_ticker.high_price.to_f64().unwrap_or(0.0);
                let low = bybit_ticker.low_price.to_f64().unwrap_or(0.0);
                if low > 0.0 {
                    (high - low) / low * 100.0
                } else {
                    0.0
                }
            } else {
                0.0
            };

            candidates.push(CoinCandidate {
                coin: coin.to_string(),
                volume_1h_usdt,
                volatility_24h_pct,
            });
        }

        // 9. 변동성 내림차순 정렬 → 상위 N개
        candidates.sort_by(|a, b| {
            b.volatility_24h_pct
                .partial_cmp(&a.volatility_24h_pct)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        candidates.truncate(max_coins);

        info!(selected = candidates.len(), "코인 선택 완료");
        for (i, c) in candidates.iter().enumerate() {
            info!(
                rank = i + 1,
                coin = c.coin,
                volume_1h_usdt = format!("{:.0}", c.volume_1h_usdt),
                volatility_24h_pct = format!("{:.2}", c.volatility_24h_pct),
                "선택된 코인"
            );
        }

        Ok(candidates)
    }
}

/// Upbit 티커에서 코인 심볼 집합을 추출합니다.
/// "KRW-BTC" → "BTC" (KRW 마켓만 대상)
fn extract_coins_upbit(tickers: &[Ticker]) -> HashSet<&str> {
    tickers
        .iter()
        .filter_map(|t| t.market.strip_prefix("KRW-"))
        .collect()
}

/// Bybit 티커에서 코인 심볼 집합을 추출합니다.
/// "BTCUSDT" → "BTC" (USDT 마켓만 대상)
fn extract_coins_bybit(tickers: &[Ticker]) -> HashSet<&str> {
    tickers
        .iter()
        .filter_map(|t| t.market.strip_suffix("USDT"))
        .collect()
}

/// Upbit 티커를 코인 심볼 → Ticker 맵으로 변환합니다.
fn build_ticker_map_upbit(tickers: &[Ticker]) -> std::collections::HashMap<&str, &Ticker> {
    tickers
        .iter()
        .filter_map(|t| t.market.strip_prefix("KRW-").map(|coin| (coin, t)))
        .collect()
}

/// Bybit 티커를 코인 심볼 → Ticker 맵으로 변환합니다.
fn build_ticker_map_bybit(tickers: &[Ticker]) -> std::collections::HashMap<&str, &Ticker> {
    tickers
        .iter()
        .filter_map(|t| t.market.strip_suffix("USDT").map(|coin| (coin, t)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use arb_exchange::error::{ExchangeError, ExchangeResult};
    use arb_exchange::types::{Candle, OrderBook, PriceChange, Ticker};
    use chrono::Utc;
    use rust_decimal::Decimal;
    use std::collections::HashMap;
    use std::str::FromStr;
    use std::sync::Mutex;

    /// 테스트용 Decimal 생성 헬퍼 (문자열에서 파싱).
    fn d(s: &str) -> Decimal {
        Decimal::from_str(s).unwrap()
    }

    /// 테스트용 MockMarketData 구현.
    struct MockMarketData {
        name: String,
        /// 전종목 티커 목록.
        tickers: Vec<Ticker>,
        /// 마켓 코드 → 캔들 데이터 매핑.
        candles: Mutex<HashMap<String, Vec<Candle>>>,
    }

    impl MockMarketData {
        fn new_upbit(tickers: Vec<Ticker>, candles: HashMap<String, Vec<Candle>>) -> Self {
            Self {
                name: "MockUpbit".to_string(),
                tickers,
                candles: Mutex::new(candles),
            }
        }

        fn new_bybit(tickers: Vec<Ticker>, candles: HashMap<String, Vec<Candle>>) -> Self {
            Self {
                name: "MockBybit".to_string(),
                tickers,
                candles: Mutex::new(candles),
            }
        }
    }

    impl MarketData for MockMarketData {
        fn name(&self) -> &str {
            &self.name
        }

        async fn get_ticker(&self, markets: &[&str]) -> ExchangeResult<Vec<Ticker>> {
            Ok(self
                .tickers
                .iter()
                .filter(|t| markets.contains(&t.market.as_str()))
                .cloned()
                .collect())
        }

        async fn get_orderbook(
            &self,
            _market: &str,
            _depth: Option<u32>,
        ) -> ExchangeResult<OrderBook> {
            Err(ExchangeError::Unsupported("orderbook not mocked".into()))
        }

        async fn get_candles(
            &self,
            market: &str,
            _interval: CandleInterval,
            _count: u32,
        ) -> ExchangeResult<Vec<Candle>> {
            let lock = self.candles.lock().unwrap();
            match lock.get(market) {
                Some(c) => Ok(c.clone()),
                None => Err(ExchangeError::MarketNotFound(format!(
                    "no candle for {market}"
                ))),
            }
        }

        async fn get_candles_before(
            &self,
            _market: &str,
            _interval: CandleInterval,
            _count: u32,
            _before: chrono::DateTime<Utc>,
        ) -> ExchangeResult<Vec<Candle>> {
            Err(ExchangeError::Unsupported(
                "candles_before not mocked".into(),
            ))
        }

        async fn get_all_tickers(&self) -> ExchangeResult<Vec<Ticker>> {
            Ok(self.tickers.clone())
        }

        fn market_code(base: &str, quote: &str) -> String {
            // 런타임에 format을 알 수 없으므로 quote로 판별
            if quote == "KRW" {
                format!("KRW-{base}")
            } else {
                format!("{base}{quote}")
            }
        }
    }

    /// 테스트용 Ticker 생성 헬퍼.
    fn make_ticker(
        market: &str,
        trade_price: Decimal,
        acc_trade_price_24h: Decimal,
        high: Decimal,
        low: Decimal,
    ) -> Ticker {
        Ticker {
            market: market.to_string(),
            trade_price,
            opening_price: trade_price,
            high_price: high,
            low_price: low,
            prev_closing_price: trade_price,
            change: PriceChange::Even,
            change_rate: Decimal::ZERO,
            change_price: Decimal::ZERO,
            acc_trade_volume_24h: Decimal::from(1000),
            acc_trade_price_24h,
            timestamp: Utc::now(),
        }
    }

    /// 테스트용 Candle 생성 헬퍼.
    fn make_candle(market: &str, close: Decimal, volume: Decimal) -> Candle {
        Candle {
            market: market.to_string(),
            timestamp: Utc::now(),
            open: close,
            high: close,
            low: close,
            close,
            volume,
        }
    }

    /// 표준 테스트 데이터 세트를 생성합니다.
    /// 코인: BTC, ETH, XRP, SOL, USDT, USDC
    fn setup_standard_data() -> (MockMarketData, MockMarketData) {
        let upbit_tickers = vec![
            make_ticker(
                "KRW-BTC",
                d("90000000"),
                d("500000000000"),
                d("91000000"),
                d("89000000"),
            ),
            make_ticker(
                "KRW-ETH",
                d("4500000"),
                d("200000000000"),
                d("4600000"),
                d("4400000"),
            ),
            make_ticker(
                "KRW-XRP",
                d("1500"),
                d("100000000000"),
                d("1600"),
                d("1400"),
            ),
            make_ticker(
                "KRW-SOL",
                d("200000"),
                d("80000000000"),
                d("210000"),
                d("190000"),
            ),
            make_ticker(
                "KRW-USDT",
                d("1350"),
                d("50000000000"),
                d("1355"),
                d("1345"),
            ),
            make_ticker(
                "KRW-USDC",
                d("1350"),
                d("30000000000"),
                d("1355"),
                d("1345"),
            ),
        ];

        let bybit_tickers = vec![
            make_ticker(
                "BTCUSDT",
                d("67000"),
                d("5000000000"),
                d("68000"),
                d("66000"),
            ),
            make_ticker("ETHUSDT", d("3300"), d("2000000000"), d("3400"), d("3200")),
            make_ticker("XRPUSDT", d("1.1"), d("800000000"), d("1.2"), d("1.0")),
            make_ticker("SOLUSDT", d("150"), d("600000000"), d("160"), d("140")),
            make_ticker("USDTUSDT", d("1"), d("100000000"), d("1"), d("1")),
            make_ticker("USDCUSDT", d("1"), d("100000000"), d("1"), d("1")),
        ];

        let upbit_candles: HashMap<String, Vec<Candle>> = HashMap::from([
            (
                "KRW-BTC".into(),
                vec![make_candle("KRW-BTC", d("90000000"), d("100"))],
            ),
            (
                "KRW-ETH".into(),
                vec![make_candle("KRW-ETH", d("4500000"), d("5000"))],
            ),
            (
                "KRW-XRP".into(),
                vec![make_candle("KRW-XRP", d("1500"), d("50000000"))],
            ),
            (
                "KRW-SOL".into(),
                vec![make_candle("KRW-SOL", d("200000"), d("100000"))],
            ),
        ]);

        let bybit_candles: HashMap<String, Vec<Candle>> = HashMap::from([
            (
                "BTCUSDT".into(),
                vec![make_candle("BTCUSDT", d("67000"), d("100"))],
            ),
            (
                "ETHUSDT".into(),
                vec![make_candle("ETHUSDT", d("3300"), d("5000"))],
            ),
            (
                "XRPUSDT".into(),
                vec![make_candle("XRPUSDT", d("1.1"), d("50000000"))],
            ),
            (
                "SOLUSDT".into(),
                vec![make_candle("SOLUSDT", d("150"), d("100000"))],
            ),
        ]);

        let upbit = MockMarketData::new_upbit(upbit_tickers, upbit_candles);
        let bybit = MockMarketData::new_bybit(bybit_tickers, bybit_candles);

        (upbit, bybit)
    }

    #[tokio::test]
    async fn test_stablecoin_excluded() {
        let (upbit, bybit) = setup_standard_data();
        let selector = CoinSelector::new(&upbit, &bybit);

        let result = selector
            .select(10, Decimal::ZERO, &[])
            .await
            .expect("select 실패");

        // USDT, USDC가 결과에 포함되지 않아야 함
        let coins: Vec<&str> = result.iter().map(|c| c.coin.as_str()).collect();
        assert!(
            !coins.contains(&"USDT"),
            "USDT가 결과에 포함되어서는 안 됩니다"
        );
        assert!(
            !coins.contains(&"USDC"),
            "USDC가 결과에 포함되어서는 안 됩니다"
        );
    }

    #[tokio::test]
    async fn test_blacklist_excluded() {
        let (upbit, bybit) = setup_standard_data();
        let selector = CoinSelector::new(&upbit, &bybit);

        let blacklist = vec!["BTC".to_string(), "ETH".to_string()];
        let result = selector
            .select(10, Decimal::ZERO, &blacklist)
            .await
            .expect("select 실패");

        let coins: Vec<&str> = result.iter().map(|c| c.coin.as_str()).collect();
        assert!(
            !coins.contains(&"BTC"),
            "블랙리스트 BTC가 제외되어야 합니다"
        );
        assert!(
            !coins.contains(&"ETH"),
            "블랙리스트 ETH가 제외되어야 합니다"
        );
    }

    #[tokio::test]
    async fn test_volume_filter() {
        let (upbit, bybit) = setup_standard_data();
        let selector = CoinSelector::new(&upbit, &bybit);

        // 매우 높은 볼륨 임계값 → 대부분 제외
        // BTC: 100 × 67000 = 6,700,000 USDT (Bybit 기준, 양쪽 중 min)
        // ETH: 5000 × 3300 = 16,500,000 USDT
        // 임계값을 10,000,000 USDT로 설정 → BTC(6.7M) 제외, ETH(16.5M) 통과
        let result = selector
            .select(10, d("10000000"), &[])
            .await
            .expect("select 실패");

        let coins: Vec<&str> = result.iter().map(|c| c.coin.as_str()).collect();
        assert!(
            !coins.contains(&"BTC"),
            "BTC는 볼륨 미달로 제외되어야 합니다"
        );
    }

    #[tokio::test]
    async fn test_volatility_sort() {
        let (upbit, bybit) = setup_standard_data();
        let selector = CoinSelector::new(&upbit, &bybit);

        let result = selector
            .select(10, Decimal::ZERO, &[])
            .await
            .expect("select 실패");

        // 변동성 내림차순 정렬 확인
        for i in 1..result.len() {
            assert!(
                result[i - 1].volatility_24h_pct >= result[i].volatility_24h_pct,
                "변동성이 내림차순이어야 합니다: {} ({:.2}%) >= {} ({:.2}%)",
                result[i - 1].coin,
                result[i - 1].volatility_24h_pct,
                result[i].coin,
                result[i].volatility_24h_pct
            );
        }
    }

    #[tokio::test]
    async fn test_max_coins_limit() {
        let (upbit, bybit) = setup_standard_data();
        let selector = CoinSelector::new(&upbit, &bybit);

        let result = selector
            .select(2, Decimal::ZERO, &[])
            .await
            .expect("select 실패");

        assert!(
            result.len() <= 2,
            "max_coins=2 이하여야 합니다, 실제: {}",
            result.len()
        );
    }

    #[tokio::test]
    async fn test_select_empty_intersection() {
        // Upbit에만 있는 코인, Bybit에만 있는 코인 → 교집합 없음
        let upbit_tickers = vec![make_ticker(
            "KRW-AAA",
            d("100"),
            d("1000000"),
            d("110"),
            d("90"),
        )];
        let bybit_tickers = vec![make_ticker(
            "BBBUSDT",
            d("100"),
            d("1000000"),
            d("110"),
            d("90"),
        )];

        let upbit = MockMarketData::new_upbit(upbit_tickers, HashMap::new());
        let bybit = MockMarketData::new_bybit(bybit_tickers, HashMap::new());
        let selector = CoinSelector::new(&upbit, &bybit);

        let result = selector
            .select(10, Decimal::ZERO, &[])
            .await
            .expect("select 실패");

        assert!(
            result.is_empty(),
            "교집합이 없으면 빈 결과를 반환해야 합니다"
        );
    }

    #[tokio::test]
    async fn test_select_fewer_than_max() {
        // 2개 코인만 존재하는 데이터 (스테이블코인 제외 후)
        let upbit_tickers = vec![
            make_ticker(
                "KRW-BTC",
                d("90000000"),
                d("500000000000"),
                d("91000000"),
                d("89000000"),
            ),
            make_ticker(
                "KRW-ETH",
                d("4500000"),
                d("200000000000"),
                d("4600000"),
                d("4400000"),
            ),
        ];
        let bybit_tickers = vec![
            make_ticker(
                "BTCUSDT",
                d("67000"),
                d("5000000000"),
                d("68000"),
                d("66000"),
            ),
            make_ticker("ETHUSDT", d("3300"), d("2000000000"), d("3400"), d("3200")),
        ];

        let upbit_candles: HashMap<String, Vec<Candle>> = HashMap::from([
            (
                "KRW-BTC".into(),
                vec![make_candle("KRW-BTC", d("90000000"), d("100"))],
            ),
            (
                "KRW-ETH".into(),
                vec![make_candle("KRW-ETH", d("4500000"), d("5000"))],
            ),
        ]);
        let bybit_candles: HashMap<String, Vec<Candle>> = HashMap::from([
            (
                "BTCUSDT".into(),
                vec![make_candle("BTCUSDT", d("67000"), d("100"))],
            ),
            (
                "ETHUSDT".into(),
                vec![make_candle("ETHUSDT", d("3300"), d("5000"))],
            ),
        ]);

        let upbit = MockMarketData::new_upbit(upbit_tickers, upbit_candles);
        let bybit = MockMarketData::new_bybit(bybit_tickers, bybit_candles);
        let selector = CoinSelector::new(&upbit, &bybit);

        // max_coins=10이지만 후보가 2개 이하
        let result = selector
            .select(10, Decimal::ZERO, &[])
            .await
            .expect("select 실패");

        // 24h 거래대금 하위 50% 제거 시 2개 중 1개만 남을 수 있음
        assert!(
            result.len() <= 2,
            "후보보다 많은 결과를 반환해서는 안 됩니다: {}",
            result.len()
        );
        // 결과가 있으면 유효한 코인이어야 함
        for c in &result {
            assert!(
                c.coin == "BTC" || c.coin == "ETH",
                "예상치 못한 코인: {}",
                c.coin
            );
        }
    }
}
