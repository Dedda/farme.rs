-- Your SQL goes here
CREATE TYPE farm_admin_status AS ENUM('NO', 'YES', 'REQUESTED');
ALTER TABLE users
    ALTER COLUMN farmowner
        DROP DEFAULT;
ALTER TABLE users
    ALTER COLUMN farmowner
        TYPE farm_admin_status
        USING (
            CASE
                WHEN farmowner = 0 THEN 'NO'
                ELSE 'YES'
            END
        )::farm_admin_status;
ALTER TABLE users
    ALTER COLUMN farmowner
        SET DEFAULT 'NO';
