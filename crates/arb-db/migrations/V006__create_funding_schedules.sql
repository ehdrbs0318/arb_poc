CREATE TABLE funding_schedules (
    id                  BIGINT AUTO_INCREMENT PRIMARY KEY,
    coin                VARCHAR(20) NOT NULL,
    interval_hours      INT NOT NULL,
    next_funding_time   DATETIME(3) NOT NULL,
    current_rate        DOUBLE NOT NULL,
    updated_at          DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3),
    UNIQUE INDEX idx_coin (coin)
);
