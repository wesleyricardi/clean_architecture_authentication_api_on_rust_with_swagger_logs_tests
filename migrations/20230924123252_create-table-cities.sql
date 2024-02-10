-- CreateTable
CREATE TABLE IF NOT EXISTS "cities" (
    "id" SERIAL NOT NULL,
    "name" TEXT NOT NULL,
    "state_id" INTEGER NOT NULL,
    CONSTRAINT "cities_state_id_fkey" FOREIGN KEY ("state_id") 
      REFERENCES "states"("id") 
      ON DELETE RESTRICT 
      ON UPDATE CASCADE
);

-- CreateIndex
CREATE UNIQUE INDEX IF NOT EXISTS "cities_id_key" 
  ON "cities"("id");

-- CreateIndex
CREATE INDEX IF NOT EXISTS "cities_id_idx" 
  ON "cities"("id");

-- CreateIndex
CREATE UNIQUE INDEX IF NOT EXISTS "cities_state_id_name_key" 
  ON "cities"("state_id", "name");