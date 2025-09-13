-- Your SQL goes here
CREATE TABLE farms (
    id SERIAL NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE geolocations (
    id SERIAL NOT NULL PRIMARY KEY,
    lat REAL NOT NULL,
    lon REAL NOT NULL
);

CREATE TABLE farm_locations (
    id SERIAL NOT NULL PRIMARY KEY,
    farm_id INTEGER UNIQUE NOT NULL ,
    location_id INTEGER NOT NULL,
    FOREIGN KEY (farm_id) REFERENCES farms(id) ON DELETE CASCADE,
    FOREIGN KEY (location_id) REFERENCES geolocations(id) ON DELETE CASCADE
);

CREATE TABLE opening_hours (
    id SERIAL NOT NULL PRIMARY KEY,
    farm_id INTEGER NOT NULL,
    weekday INTEGER NOT NULL,
    open TIME NOT NULL,
    close TIME NOT NULL,
    FOREIGN KEY (farm_id) REFERENCES farms(id) ON DELETE CASCADE
);

CREATE TABLE shop_types (
    id SERIAL NOT NULL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE farm_shop_types (
    id SERIAL NOT NULL PRIMARY KEY,
    farm_id INTEGER NOT NULL,
    shop_type_id INTEGER NOT NULL,
    FOREIGN KEY (farm_id) REFERENCES farms(id) ON DELETE CASCADE,
    FOREIGN KEY (shop_type_id) REFERENCES shop_types(id) ON DELETE CASCADE,
    UNIQUE (farm_id, shop_type_id)
);

CREATE TABLE contact (
    id SERIAL NOT NULL PRIMARY KEY,
    farm_id INTEGER NOT NULL,
    email TEXT,
    phone TEXT,
    address TEXT,
    FOREIGN KEY (farm_id) REFERENCES farms(id) ON DELETE CASCADE
);

INSERT INTO shop_types(id, name) VALUES (1, 'store') ON CONFLICT DO NOTHING;
INSERT INTO shop_types(id, name) VALUES (2, 'self-service') ON CONFLICT DO NOTHING;
INSERT INTO shop_types(id, name) VALUES (3, 'vending machine') ON CONFLICT DO NOTHING;
