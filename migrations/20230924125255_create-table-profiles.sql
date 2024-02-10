CREATE TABLE IF NOT EXISTS profiles
(
    id TEXT NOT NULL,
    name TEXT NOT NULL,
    username TEXT NOT NULL,
    birth_date DATE NOT NULL,
    gender_id INTEGER NOT NULL,
    address_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT profiles_address_id_fkey FOREIGN KEY (address_id)
        REFERENCES addresses (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT,
    CONSTRAINT profiles_gender_id_fkey FOREIGN KEY (gender_id)
        REFERENCES genders (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT,
    CONSTRAINT profiles_user_id_fkey FOREIGN KEY (user_id)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT
);

CREATE UNIQUE INDEX IF NOT EXISTS profiles_id_key
    ON profiles (id);

CREATE UNIQUE INDEX IF NOT EXISTS profiles_user_id_key
    ON profiles (user_id);


CREATE UNIQUE INDEX IF NOT EXISTS profiles_username_key
    ON profiles (username);


CREATE TRIGGER "tr_profiles_update_updatedAt"
    BEFORE UPDATE 
    ON profiles
    FOR EACH ROW
    EXECUTE FUNCTION update_updatedat();