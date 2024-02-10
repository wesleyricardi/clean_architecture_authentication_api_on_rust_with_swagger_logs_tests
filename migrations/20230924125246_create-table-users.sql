-- CreateTable
CREATE TABLE IF NOT EXISTS "users" (
    "id" SERIAL NOT NULL,
    "password" TEXT NOT NULL,
    "account_disabled" BOOLEAN NOT NULL DEFAULT false,
    "blocked_by_attempts" BOOLEAN NOT NULL DEFAULT false,
    "requested_deletion" BOOLEAN NOT NULL DEFAULT false,
    "sign_in_id" INTEGER NOT NULL,
    "request_deletion_at" TIMESTAMP(3),
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "users_sign_in_id_fkey" FOREIGN KEY ("sign_in_id") 
      REFERENCES "sign_ins"("id") 
      ON DELETE RESTRICT 
      ON UPDATE CASCADE
);

-- CreateIndex
CREATE UNIQUE INDEX IF NOT EXISTS "users_id_key" 
  ON "users"("id");

-- CreateIndex
CREATE UNIQUE INDEX IF NOT EXISTS "users_sign_in_id_key" 
  ON "users"("sign_in_id");

-- CreateIndex
CREATE INDEX IF NOT EXISTS "users_id_idx" 
  ON "users"("id");

-- Create Trigger
CREATE OR REPLACE TRIGGER "tr_users_update_updatedAt"
  BEFORE UPDATE ON "users"
  FOR EACH ROW
  EXECUTE FUNCTION update_updatedAt();