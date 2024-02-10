-- CreateTable
CREATE TABLE IF NOT EXISTS "emails" (
    "id" SERIAL NOT NULL,
    "address" TEXT NOT NULL,
    "checked" BOOLEAN NOT NULL DEFAULT false,
    "profile_id" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "checked_at" TIMESTAMP(3),
    CONSTRAINT "emails_profile_id_fkey" FOREIGN KEY ("profile_id") 
      REFERENCES "profiles"("id") 
      ON DELETE RESTRICT 
      ON UPDATE CASCADE
);

-- CreateIndex
CREATE UNIQUE INDEX IF NOT EXISTS "emails_id_key" 
  ON "emails"("id");

-- CreateIndex
CREATE UNIQUE INDEX IF NOT EXISTS "emails_address_key" 
  ON "emails"("address");

-- CreateIndex
CREATE INDEX IF NOT EXISTS "emails_address_idx" 
  ON "emails"("address");