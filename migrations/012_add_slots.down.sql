ALTER TABLE teams
    DROP COLUMN reserved_slot;

DROP TABLE IF EXISTS reservations;
DROP TABLE IF EXISTS slots;
