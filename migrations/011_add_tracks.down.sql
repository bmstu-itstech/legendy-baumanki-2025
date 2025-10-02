DROP TABLE IF EXISTS answers;
DROP TABLE IF EXISTS team_started_tracks;
DROP TABLE IF EXISTS task_correct_answers;
DROP TABLE IF EXISTS task_dependencies;
DROP TABLE IF EXISTS task_options;
DROP TABLE IF EXISTS tasks;
DROP TABLE IF EXISTS tracks;
DROP TYPE  IF EXISTS TASK_TYPE;
DROP TYPE  IF EXISTS TRACK_TAG;

DO $$ BEGIN
    CREATE TYPE TASK_TYPE AS ENUM (
        'rebus',
        'riddle'
    );
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS tasks (
    id                          VARCHAR(6)  PRIMARY KEY,
    index                       INTEGER     NOT NULL,
    task_type                   TASK_TYPE   NOT NULL,
    media_id                    VARCHAR(64) NOT NULL,
    explanation                 VARCHAR     NOT NULL,
    correct_answer              VARCHAR     NOT NULL,
    points                      INTEGER     NOT NULL,
    max_levenshtein_distance    INTEGER     NOT NULL CHECK(max_levenshtein_distance >= 0)
);

ALTER TABLE tasks
    ADD CONSTRAINT fk_task_media
        FOREIGN KEY (media_id)
            REFERENCES media (id);

CREATE TABLE IF NOT EXISTS answers (
    id              VARCHAR(8)  PRIMARY KEY,
    task_id         VARCHAR(6)  NOT NULL,
    user_id         BIGINT      NOT NULL,
    text            VARCHAR     NOT NULL,
    points          INTEGER     NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

ALTER TABLE answers
    ADD CONSTRAINT fk_answer_tasks
        FOREIGN KEY (task_id)
            REFERENCES tasks (id);

ALTER TABLE answers
    ADD CONSTRAINT fk_answer_users
        FOREIGN KEY (user_id)
            REFERENCES users (id);
