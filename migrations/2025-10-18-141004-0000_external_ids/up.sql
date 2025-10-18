-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
ALTER TABLE users ADD COLUMN ext_id UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4();
ALTER TABLE farms ADD COLUMN ext_id UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4();
