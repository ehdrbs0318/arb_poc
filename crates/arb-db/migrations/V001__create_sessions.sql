CREATE TABLE sessions (
    id                  BIGINT AUTO_INCREMENT PRIMARY KEY,
    parent_session_id   BIGINT UNSIGNED NULL,
    started_at          DATETIME(3) NOT NULL,
    ended_at            DATETIME(3),
    config_json         TEXT NOT NULL,
    status              VARCHAR(20) NOT NULL
);
