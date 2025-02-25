CREATE TABLE user_dns_records (
    id INTEGER PRIMARY KEY,
    domain_name TEXT NOT NULL,
    record_type INTEGER NOT NULL,
    record_value BLOB NOT NULL,
    ttl INTEGER NOT NULL DEFAULT 3600,
    priority INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN NOT NULL DEFAULT 1
);
