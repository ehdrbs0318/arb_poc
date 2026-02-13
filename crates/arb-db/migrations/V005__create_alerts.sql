CREATE TABLE alerts (
    id              BIGINT AUTO_INCREMENT PRIMARY KEY,
    session_id      BIGINT NOT NULL,
    level           VARCHAR(20) NOT NULL,
    event_type      VARCHAR(50) NOT NULL,
    message         TEXT NOT NULL,
    payload_json    TEXT,
    created_at      DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    INDEX idx_session_level (session_id, level)
);
