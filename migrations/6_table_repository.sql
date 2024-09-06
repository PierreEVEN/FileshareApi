CREATE TABLE IF NOT EXISTS SCHEMA_NAME.repository (
        id BIGSERIAL PRIMARY KEY,
        name VARCHAR(200) UNIQUE NOT NULL,
        owner BIGSERIAL NOT NULL,
        description TEXT,
        status SCHEMA_NAME.repository_status DEFAULT 'hidden' NOT NULL,
        display_name VARCHAR(200) NOT NULL,
        max_file_size BIGINT DEFAULT 1048576000,
        visitor_file_lifetime int,
        allow_visitor_upload BOOLEAN DEFAULT false NOT NULL,
        FOREIGN KEY(owner) REFERENCES SCHEMA_NAME.users(id)
    );