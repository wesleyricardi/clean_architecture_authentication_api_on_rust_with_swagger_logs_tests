DROP FUNCTION IF EXISTS sign_in_by;

CREATE OR REPLACE FUNCTION sign_in_by(field TEXT, value TEXT)
RETURNS TABLE (
  id TEXT,
  name TEXT,
  username TEXT,
  password TEXT
) AS $$
BEGIN
  RETURN QUERY EXECUTE 
    'SELECT 
      profiles.id, 
      profiles.name, 
      profiles.username,
      users.password
    FROM 
      profiles
    JOIN
      users ON profiles.user_id = users.id
    LEFT JOIN 
      telephones ON profiles.id = telephones.profile_id
    LEFT JOIN 
      emails ON profiles.id = emails.profile_id
    WHERE ' || field || ' = $1
    GROUP BY
      profiles.id, 
      profiles.name, 
      profiles.username, 
      users.password'
    USING value;
END;
$$ LANGUAGE plpgsql;