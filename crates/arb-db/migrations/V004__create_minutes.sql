CREATE TABLE minutes (
    id              BIGINT AUTO_INCREMENT PRIMARY KEY,
    session_id      BIGINT NOT NULL,
    coin            VARCHAR(20) NOT NULL,
    ts              DATETIME(3) NOT NULL,
    upbit_close     DECIMAL(20,4),
    bybit_close     DECIMAL(20,8),
    spread_pct      DOUBLE,
    z_score         DOUBLE,
    mean            DOUBLE,
    stddev          DOUBLE,
    INDEX idx_session_coin_ts (session_id, coin, ts)
);
