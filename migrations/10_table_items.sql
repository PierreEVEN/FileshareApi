CREATE TABLE IF NOT EXISTS SCHEMA_NAME.items (
        id BIGSERIAL PRIMARY KEY,
        repository BIGSERIAL NOT NULL,
        owner BIGSERIAL NOT NULL,
        name VARCHAR(200) NOT NULL,
        is_regular_file BOOLEAN NOT NULL,
        description TEXT,
        parent_item BIGINT NULL,
        absolute_path VARCHAR UNIQUE DEFAULT NULL,
        in_trash BOOLEAN DEFAULT FALSE NOT NULL,
        FOREIGN KEY(repository) REFERENCES SCHEMA_NAME.repository(id),
        FOREIGN KEY(owner) REFERENCES SCHEMA_NAME.users(id),
        FOREIGN KEY(parent_item) REFERENCES SCHEMA_NAME.items(id)
    );