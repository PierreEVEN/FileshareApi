CREATE TABLE IF NOT EXISTS SCHEMA_NAME.subscriptions (
        owner BIGSERIAL,
        repository BIGSERIAL,
        root_item BIGSERIAL,
        access_type SCHEMA_NAME.user_access NOT NULL DEFAULT 'read-only',
        PRIMARY KEY(OWNER, repository, root_item),
        FOREIGN KEY(owner) REFERENCES SCHEMA_NAME.users(id),
        FOREIGN KEY(repository) REFERENCES SCHEMA_NAME.repository(id)
    );