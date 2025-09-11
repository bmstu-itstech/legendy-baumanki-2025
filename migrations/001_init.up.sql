CREATE TABLE users (
    id          BIGINT      PRIMARY KEY,
    username    VARCHAR                     DEFAULT NULL,
    full_name   VARCHAR     NOT NULL,
    group_name  VARCHAR     NOT NULL,
    team_id     VARCHAR(6)                  DEFAULT NULL,
    created_at  TIMESTAMPTZ NOT NULL        DEFAULT now()
);

CREATE TABLE teams (
    id      VARCHAR(6)  PRIMARY KEY,
    name    VARCHAR     NOT NULL,
    captain_id BIGINT   NOT NULL
);

ALTER TABLE users
    ADD CONSTRAINT fk_users_teams 
        FOREIGN KEY (team_id) 
        REFERENCES  teams (id)
        ON DELETE SET NULL;

ALTER TABLE teams
    ADD CONSTRAINT fk_teams_users_captain 
        FOREIGN KEY (captain_id) 
        REFERENCES  users (id)
        ON DELETE RESTRICT;
