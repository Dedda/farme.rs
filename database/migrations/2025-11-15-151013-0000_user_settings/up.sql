-- Your SQL goes here
CREATE TABLE user_settings (
    id SERIAL NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    setting_name TEXT NOT NULL,
    setting_value TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE (user_id, setting_name)
);
