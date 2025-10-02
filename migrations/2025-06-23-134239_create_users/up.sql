-- Your SQL goes here
CREATE TABLE users (
    id SERIAL NOT NULL PRIMARY KEY,
    firstname TEXT NOT NULL,
    lastname TEXT NOT NULL,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    sysadmin INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE farm_admins (
    id SERIAL NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    farm_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (farm_id) REFERENCES  farms(id)
);
