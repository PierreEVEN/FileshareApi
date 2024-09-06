CREATE TABLE IF NOT EXISTS SCHEMA_NAME.users (
        id BIGSERIAL PRIMARY KEY,
        email VARCHAR(200) UNIQUE,
        name VARCHAR(200) UNIQUE,
        password_hash VARCHAR(64),
        allow_contact BOOLEAN DEFAULT false NOT NULL,
        role SCHEMA_NAME.user_role DEFAULT 'guest' NOT NULL
    );