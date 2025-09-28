CREATE TABLE IF NOT EXISTS feedbacks (
    id          INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    author_id   BIGINT NOT NULL,
    text        VARCHAR NOT NULL
);

ALTER TABLE feedbacks
    ADD CONSTRAINT fk_feedback_users
        FOREIGN KEY (author_id)
        REFERENCES users (id);
