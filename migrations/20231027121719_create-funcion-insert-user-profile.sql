DROP FUNCTION IF EXISTS insert_user_profile;

CREATE FUNCTION insert_user_profile(
    profile_id VARCHAR,
    profile_name VARCHAR, 
    profile_username VARCHAR, 
    profile_birth_date DATE, 
    profile_gender_id INT, 
    user_password VARCHAR, 
    address_street VARCHAR, 
    address_neighborhood VARCHAR, 
    address_city_id INT, 
    address_postal_code INT,
    email_address VARCHAR,
    telephone_number VARCHAR DEFAULT NULL
)
RETURNS INT AS $$
DECLARE
    profile_address_id INT;
    user_sign_in_id INT;
    profile_user_id INT;
BEGIN
    SELECT id INTO profile_address_id
    FROM addresses
    WHERE street = address_street
      AND neighborhood = address_neighborhood
      AND city_id = address_city_id;

    IF profile_address_id IS NULL THEN
        INSERT INTO addresses (street, neighborhood, city_id, postal_code) 
        VALUES (address_street, address_neighborhood, address_city_id, address_postal_code) 
        RETURNING id INTO profile_address_id;
    END IF;

    INSERT INTO sign_ins (sign_in_count) 
    VALUES (0) 
    RETURNING id INTO user_sign_in_id;

    INSERT INTO users (password, sign_in_id) 
    VALUES (user_password, user_sign_in_id) 
    RETURNING id INTO profile_user_id;

    INSERT INTO profiles (id, name, username, birth_date, gender_id, address_id, user_id) 
    VALUES (profile_id, profile_name, profile_username, profile_birth_date, profile_gender_id, profile_address_id, profile_user_id);

    INSERT INTO emails (address, profile_id) VALUES (email_address, profile_id);

    IF telephone_number IS NOT NULL THEN
      INSERT INTO telephones (number, profile_id) VALUES (telephone_number, profile_id);
    END IF;

    RETURN profile_user_id;
END;
$$ LANGUAGE plpgsql;