ALTER TABLE IF EXISTS character_facts
    DROP CONSTRAINT IF EXISTS fk_character_facts_character;

DROP TABLE IF EXISTS character_facts;
DROP TABLE IF EXISTS characters;
