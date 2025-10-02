ALTER TABLE teams
    ADD COLUMN IF NOT EXISTS hint_points INTEGER NOT NULL DEFAULT 0;

CREATE TEMP TABLE IF NOT EXISTS team_hint_points (
    team_id     VARCHAR(6)  PRIMARY KEY,
    hint_points INTEGER     NOT NULL
);

INSERT INTO
    team_hint_points (
        team_id,
        hint_points
    )
SELECT
    t.id,
    SUM(u.user_total) AS total
FROM teams t
    JOIN (
        SELECT
            u.id,
            u.team_id,
            SUM(a.points) AS user_total
        FROM users u
        JOIN
            answers a
        ON a.user_id = u.id
        GROUP BY u.id
    ) u
    ON u.team_id = t.id
GROUP BY t.id;

UPDATE teams
SET hint_points = COALESCE(
    (
        SELECT t.hint_points
        FROM team_hint_points t
        WHERE t.team_id = id
    ),
    0
)
WHERE true;
