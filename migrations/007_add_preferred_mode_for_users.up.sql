DO $$ BEGIN
    CREATE TYPE PARTICIPATION_MODE AS ENUM (
        'solo',
        'want_team',
        'team'
    );
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

ALTER TABLE users
    ADD COLUMN IF NOT EXISTS mode PARTICIPATION_MODE NOT NULL DEFAULT 'want_team';

UPDATE users
SET mode = 'team'
WHERE
    team_id IS NOT NULL;
