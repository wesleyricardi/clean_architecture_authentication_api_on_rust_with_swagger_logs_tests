-- Create function
CREATE OR REPLACE FUNCTION update_updatedAt()
  RETURNS TRIGGER AS $$
  BEGIN
    NEW.updated_at = now();
    RETURN NEW;
  END;
  $$ LANGUAGE plpgsql;