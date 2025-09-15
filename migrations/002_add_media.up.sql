DO $$ BEGIN
    CREATE TYPE MEDIA_TYPE AS ENUM (
        'image',
        'video_note'
    );
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS media (
    id          VARCHAR(64) PRIMARY KEY,
    file_id     VARCHAR     NOT NULL,
    media_type  MEDIA_TYPE  NOT NULL
);
