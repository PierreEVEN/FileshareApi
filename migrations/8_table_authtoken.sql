CREATE TABLE IF NOT EXISTS SCHEMA_NAME.authtoken(
        owner BIGSERIAL,
        token VARCHAR(200) NOT NULL UNIQUE,
        device VARCHAR(255) NOT NULL,
        expdate BIGINT NOT NULL
    );