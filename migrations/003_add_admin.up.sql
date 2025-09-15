CREATE TABLE admins (
    user_id BIGINT PRIMARY KEY
);

ALTER TABLE admins
    ADD CONSTRAINT fk_admin_users
        FOREIGN KEY (user_id) REFERENCES users (id);
