-- CreateTable
CREATE TABLE IF NOT EXISTS "countries" (
  "id" SERIAL NOT NULL,
  "name" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX IF NOT EXISTS "countries_id_key" 
  ON "countries"("id");

-- CreateIndex
CREATE UNIQUE INDEX IF NOT EXISTS "countries_name_key" 
  ON "countries"("name");

-- CreateIndex
CREATE INDEX IF NOT EXISTS "countries_id_idx" 
  ON "countries"("id");