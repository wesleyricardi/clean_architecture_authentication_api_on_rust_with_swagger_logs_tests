-- Add migration script here
-- CreateTable
CREATE TABLE IF NOT EXISTS "states" (
    "id" SERIAL NOT NULL,
    "name" TEXT NOT NULL,
    "country_id" INTEGER NOT NULL,
    CONSTRAINT "states_country_id_fkey" FOREIGN KEY ("country_id") 
      REFERENCES "countries"("id") 
      ON DELETE RESTRICT 
      ON UPDATE CASCADE
);

-- CreateIndex
CREATE UNIQUE INDEX IF NOT EXISTS "states_id_key" 
  ON "states"("id");

-- CreateIndex
CREATE INDEX IF NOT EXISTS "states_id_idx" 
  ON "states"("id");

-- CreateIndex
CREATE UNIQUE INDEX IF NOT EXISTS "states_country_id_name_key" 
  ON "states"("country_id", "name");