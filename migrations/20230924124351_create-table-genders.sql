-- CreateTable
CREATE TABLE IF NOT EXISTS "genders" (
    "id" SERIAL NOT NULL,
    "name" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX IF NOT EXISTS "genders_id_key" 
  ON "genders"("id");

-- CreateIndex
CREATE UNIQUE INDEX IF NOT EXISTS "genders_name_key" 
  ON "genders"("name");

-- CreateIndex
CREATE INDEX IF NOT EXISTS "genders_id_idx" 
  ON "genders"("id");