DROP VIEW IF EXISTS full_profile;

CREATE OR REPLACE VIEW "full_profile" AS
  SELECT 
  profiles.id, 
  profiles.name, 
  profiles.username, 
  profiles.birth_date, 
  genders.name AS gender, 
  jsonb_agg(
    json_build_object(
      'number', telephones.number, 
      'checked', telephones.checked
    )) AS telephones,
  jsonb_agg(
    json_build_object(
      'address', emails.address, 
      'checked', emails.checked
    )) AS emails,
  addresses.street, 
  addresses.neighborhood, 
  addresses.postal_code,
  cities.name AS city, 
  states.name AS state, 
  countries.name AS country
  FROM 
    profiles
  JOIN 
    addresses ON profiles.address_id = addresses.id
  JOIN 
    genders ON profiles.gender_id = genders.id
  JOIN 
    cities ON addresses.city_id = cities.id
  JOIN 
    states ON cities.state_id = states.id
  JOIN 
    countries ON states.country_id = countries.id
  LEFT JOIN 
    telephones ON profiles.id = telephones.profile_id
  LEFT JOIN 
    emails ON profiles.id = emails.profile_id
  GROUP BY 
    profiles.id, 
    profiles.name, 
    profiles.username, 
    profiles.birth_date, 
    genders.name, 
    addresses.street, 
    addresses.neighborhood, 
    addresses.postal_code,
    cities.name,
    states.name,
    countries.name