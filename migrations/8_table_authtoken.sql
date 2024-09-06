CREATE TABLE IF NOT EXISTS SCHEMA_NAME.authtoken(
        owner BIGSERIAL,
        token VARCHAR(200) NOT NULL UNIQUE,
        expdate BIGINT NOT NULL
    );