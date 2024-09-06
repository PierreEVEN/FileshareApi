CREATE TABLE IF NOT EXISTS SCHEMA_NAME.file (
        id BIGSERIAL PRIMARY KEY,
        size BIGINT NOT NULL,
        mimetype VARCHAR(200) NOT NULL,
        hash VARCHAR(64) NOT NULL,
        timestamp BIGINT NOT NULL,
        object BIGINT NULL,
        FOREIGN KEY(id) REFERENCES SCHEMA_NAME.items(id),
        FOREIGN KEY(object) REFERENCES SCHEMA_NAME.objects(id)
    );