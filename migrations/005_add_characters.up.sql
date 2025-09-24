CREATE TABLE IF NOT EXISTS characters (
    id          VARCHAR(4)  PRIMARY KEY,
    name        VARCHAR     NOT NULL,
    quote       VARCHAR     NOT NULL,
    legacy      VARCHAR     NOT NULL,
    media_id    VARCHAR(64) NOT NULL
);

CREATE TABLE IF NOT EXISTS character_facts (
    character_id    VARCHAR(6)  NOT NULL,
    fact            VARCHAR     NOT NULL
);

ALTER TABLE characters
    ADD CONSTRAINT fk_character_media
        FOREIGN KEY (media_id)
            REFERENCES media (id)
            ON DELETE CASCADE;

ALTER TABLE character_facts
    ADD CONSTRAINT fk_character_facts_character
        FOREIGN KEY (character_id)
        REFERENCES characters (id)
        ON DELETE CASCADE;
