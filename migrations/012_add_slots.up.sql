CREATE TABLE IF NOT EXISTS slots (
    id       VARCHAR(4)  PRIMARY KEY,
    start    TIME        NOT NULL,
    site     VARCHAR     NOT NULL,
    capacity INTEGER     NOT NULL
);

CREATE TABLE IF NOT EXISTS reservations (
    slot_id VARCHAR(4) NOT NULL REFERENCES slots (id) ON DELETE CASCADE,
    team_id VARCHAR(6) NOT NULL REFERENCES teams (id) ON DELETE CASCADE,
    places  INTEGER    NOT NULL,
    PRIMARY KEY (slot_id, team_id)
);

ALTER TABLE teams
    ADD COLUMN reserved_slot VARCHAR(6) REFERENCES slots (id) ON DELETE CASCADE DEFAULT NULL;
