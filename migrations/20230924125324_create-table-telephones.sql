-- CreateTable
CREATE TABLE IF NOT EXISTS "telephones" (
    "id" SERIAL NOT NULL,
    "number" TEXT NOT NULL,
    "checked" BOOLEAN NOT NULL DEFAULT false,
    "profile_id" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "checked_at" TIMESTAMP(3),
    CONSTRAINT "telephones_profile_id_fkey" FOREIGN KEY ("profile_id") 
      REFERENCES "profiles"("id") 
      ON DELETE RESTRICT 
      ON UPDATE CASCADE
);

-- CreateIndex
CREATE UNIQUE INDEX IF NOT EXISTS "telephones_id_key" 
  ON "telephones"("id");

-- CreateIndex
CREATE UNIQUE INDEX IF NOT EXISTS "telephones_number_key" 
  ON "telephones"("number");

-- CreateIndex
CREATE INDEX IF NOT EXISTS "telephones_number_idx" 
  ON "telephones"("number");