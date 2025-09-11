ALTER TABLE teams
    DROP CONSTRAINT fk_teams_users_captain;

ALTER TABLE users
    DROP CONSTRAINT fk_users_teams;

DROP TABLE IF EXISTS teams;
DROP TABLE IF EXISTS users;

