CREATE TABLE IF NOT EXISTS SCHEMA_NAME.directory_data (
        id BIGSERIAL PRIMARY KEY,
        open_upload BOOLEAN NOT NULL,
        FOREIGN KEY(id) REFERENCES SCHEMA_NAME.items(id)
    );