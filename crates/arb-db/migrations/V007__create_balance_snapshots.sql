CREATE TABLE balance_snapshots (
    id                 BIGINT       NOT NULL AUTO_INCREMENT PRIMARY KEY,
    created_at         DATETIME(3)  NOT NULL,
    snapshot_group_id  BIGINT       NOT NULL COMMENT '같은 트리거로 생성된 행 그룹 식별자',
    session_id         BIGINT       NOT NULL,
    record_type        VARCHAR(10)  NOT NULL COMMENT 'PERIODIC | POS_ENT | POS_EXT',
    cex                VARCHAR(10)  NOT NULL COMMENT 'UPBIT | BYBIT',
    currency           VARCHAR(10)  NOT NULL COMMENT 'KRW | USDT',
    available          DECIMAL(20,8) NOT NULL COMMENT '기축통화 주문 가능 잔고',
    locked             DECIMAL(20,8) NOT NULL COMMENT '기축통화 잠긴 잔고 (주문 중)',
    coin_value         DECIMAL(20,8) NOT NULL DEFAULT 0 COMMENT '보유 코인/포지션 환산 가치',
    total              DECIMAL(20,8) NOT NULL COMMENT '총 자산 가치',
    position_id        BIGINT       NULL     COMMENT 'POS_ENT/POS_EXT 시 positions.id FK',
    usd_krw            DOUBLE       NOT NULL COMMENT '기록 시점 USD/KRW 공시 환율',
    usdt_krw           DOUBLE       NOT NULL COMMENT '기록 시점 USDT/KRW 거래소 시세',
    total_usd          DECIMAL(20,8) NOT NULL COMMENT 'USD 환산 총자산',
    total_usdt         DECIMAL(20,8) NOT NULL COMMENT 'USDT 환산 총자산',

    INDEX idx_session_created (session_id, created_at),
    INDEX idx_session_type (session_id, record_type),
    INDEX idx_snapshot_group (snapshot_group_id),
    INDEX idx_position (position_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
