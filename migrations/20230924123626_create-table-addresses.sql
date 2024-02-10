-- CreateTable
CREATE TABLE IF NOT EXISTS "addresses" (
    "id" SERIAL NOT NULL,
    "street" TEXT NOT NULL,
    "neighborhood" TEXT NOT NULL,
    "city_id" INTEGER NOT NULL,
    "postal_code" INTEGER NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "addresses_city_id_fkey" FOREIGN KEY ("city_id") 
      REFERENCES "cities"("id") 
      ON DELETE RESTRICT 
      ON UPDATE CASCADE
);

-- CreateIndex
CREATE UNIQUE INDEX IF NOT EXISTS "addresses_id_key" 
  ON "addresses"("id");

-- CreateIndex
CREATE INDEX IF NOT EXISTS "addresses_id_idx" 
  ON "addresses"("id");

-- CreateIndex
CREATE UNIQUE INDEX IF NOT EXISTS "addresses_city_id_neighborhood_street_key" 
  ON "addresses"("city_id", "neighborhood", "street");

-- Create Trigger
CREATE OR REPLACE TRIGGER "tr_profiles_update_updatedAt"
  BEFORE UPDATE ON "addresses"
  FOR EACH ROW
  EXECUTE FUNCTION update_updatedAt();