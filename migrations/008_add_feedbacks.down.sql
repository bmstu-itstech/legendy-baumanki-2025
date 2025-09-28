ALTER TABLE feedbacks
    DROP CONSTRAINT IF EXISTS fk_feedback_users;

DROP TABLE IF EXISTS feedbacks;
