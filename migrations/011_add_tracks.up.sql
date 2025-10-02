DROP TABLE IF EXISTS answers;
DROP TABLE IF EXISTS tasks;
DROP TYPE  IF EXISTS TASK_TYPE;


DO $$ BEGIN
    CREATE TYPE TRACK_TAG AS ENUM (
        'muzhestvo',
        'volya',
        'trud',
        'uporstvo',
        'universitet'
    );
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

DO $$ BEGIN
    CREATE TYPE TASK_TYPE AS ENUM (
        'text',
        'choice',
        'photo'
    );
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS tracks (
    tag         TRACK_TAG PRIMARY KEY,
    description VARCHAR NOT NULL,
    media_id    VARCHAR(64) NOT NULL
);

CREATE TABLE IF NOT EXISTS tasks (
    id           INTEGER     PRIMARY KEY,
    track_tag    TRACK_TAG   NOT NULL        REFERENCES tracks (tag) ON DELETE CASCADE,
    task_type    TASK_TYPE   NOT NULL,
    question     VARCHAR     NOT NULL,
    explanation  VARCHAR     NOT NULL,
    media_id     VARCHAR(64) DEFAULT NULL    REFERENCES media (id) ON DELETE CASCADE ,
    points       INTEGER     NOT NULL        CHECK ( points > 0 ),
    price        INTEGER     NOT NULL        CHECK ( price > 0 ),
    max_lvnsht_d INTEGER     NOT NULL        CHECK ( max_lvnsht_d >= 0 )
);

CREATE TABLE IF NOT EXISTS task_options (
    task_id     INTEGER NOT NULL REFERENCES tasks (id) ON DELETE CASCADE,
    option      VARCHAR NOT NULL,
    PRIMARY KEY (task_id, option)
);

CREATE TABLE IF NOT EXISTS task_dependencies (
    task_id     INTEGER NOT NULL REFERENCES tasks (id) ON DELETE CASCADE,
    dependency  INTEGER NOT NULL REFERENCES tasks (id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, dependency)
);

CREATE TABLE IF NOT EXISTS task_correct_answers (
    task_id INTEGER NOT NULL REFERENCES tasks (id) ON DELETE CASCADE,
    answer VARCHAR NOT NULL,
    PRIMARY KEY (task_id, answer)
);

CREATE TABLE IF NOT EXISTS team_started_tracks (
    team_id     VARCHAR(6)  NOT NULL    REFERENCES teams (id)   ON DELETE CASCADE,
    track_tag   TRACK_TAG   NOT NULL    REFERENCES tracks (tag) ON DELETE CASCADE,
    started_at  TIMESTAMPTZ NOT NULL,
    finished_at TIMESTAMPTZ DEFAULT NULL,
    PRIMARY KEY (team_id, track_tag)
);

CREATE TABLE IF NOT EXISTS answers (
    team_id         VARCHAR(6)  NOT NULL REFERENCES teams (id) ON DELETE CASCADE,
    task_id         INTEGER     NOT NULL REFERENCES tasks (id) ON DELETE CASCADE,
    text            VARCHAR     NOT NULL,
    points          INTEGER     NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (team_id, task_id)
);
