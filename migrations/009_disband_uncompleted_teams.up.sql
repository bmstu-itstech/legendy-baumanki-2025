BEGIN;

CREATE TEMP TABLE uncompleted_teams (
    id VARCHAR(6) PRIMARY KEY
);

INSERT INTO
    uncompleted_teams (id)
SELECT team_id
FROM users
GROUP BY team_id
HAVING COUNT(id) < 5;

UPDATE users
SET team_id = NULL
WHERE
    team_id IS NOT NULL
    AND team_id IN (
        SELECT id FROM uncompleted_teams
    );

DELETE FROM teams
WHERE
    id IN (SELECT id FROM uncompleted_teams);

COMMIT;
