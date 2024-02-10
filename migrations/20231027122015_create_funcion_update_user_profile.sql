DROP FUNCTION IF EXISTS update_user_profile;

CREATE OR REPLACE FUNCTION update_user_profile(
    profile_id VARCHAR,
    profile_name VARCHAR DEFAULT NULL, 
    profile_birth_date DATE DEFAULT NULL, 
    profile_gender_id INT DEFAULT NULL, 
    address_street VARCHAR DEFAULT NULL, 
    address_neighborhood VARCHAR DEFAULT NULL, 
    address_city_id INT DEFAULT NULL, 
    postal_code INT DEFAULT NULL,
    email_address VARCHAR DEFAULT NULL,
    telephone_number VARCHAR DEFAULT NULL
)
RETURNS VARCHAR AS $$
DECLARE
    address_id INT;
BEGIN
    IF address_street IS NOT NULL 
    AND address_neighborhood IS NOT NULL 
    AND address_city_id IS NOT NULL 
    AND postal_code IS NOT NULL THEN
    
        SELECT id INTO address_id
        FROM addresses
        WHERE street = address_street
        AND neighborhood = address_neighborhood
        AND city_id = address_city_id;

        IF address_id IS NULL THEN
            INSERT INTO addresses (street, neighborhood, city_id, postal_code) 
            VALUES (address_street, address_neighborhood, address_city_id, postal_code) 
            RETURNING id INTO address_id;
        END IF;

        UPDATE profiles SET address_id = address_id WHERE id = profile_id;
    END IF;

    IF profile_name IS NOT NULL THEN
        UPDATE profiles SET name = profile_name WHERE id = profile_id;
    END IF;

    IF profile_birth_date IS NOT NULL THEN
        UPDATE profiles SET birth_date = profile_birth_date WHERE id = profile_id;
    END IF;

    IF profile_gender_id IS NOT NULL THEN
        UPDATE profiles SET gender_id = profile_gender_id WHERE id = profile_id;
    END IF;

    IF email_address IS NOT NULL THEN
        INSERT INTO emails (address, profile_id) VALUES (email_address, profile_id);
    END IF;

    IF telephone_number IS NOT NULL THEN
        INSERT INTO telephones (number, profile_id) VALUES (telephone_number, profile_id);
    END IF;

    RETURN profile_id;
END;
$$ LANGUAGE plpgsql;