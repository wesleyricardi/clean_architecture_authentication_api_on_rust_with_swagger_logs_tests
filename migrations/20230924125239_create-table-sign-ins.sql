-- CreateTable
CREATE TABLE IF NOT EXISTS "sign_ins" (
    "id" SERIAL NOT NULL,
    "sign_in_count" INTEGER NOT NULL DEFAULT 0,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- CreateIndex
CREATE UNIQUE INDEX IF NOT EXISTS "sign_ins_id_key" 
  ON "sign_ins"("id");

-- CreateIndex
CREATE INDEX IF NOT EXISTS "sign_ins_id_idx" 
  ON "sign_ins"("id");

-- Create Trigger
CREATE OR REPLACE TRIGGER "tr_sign_ins_update_updatedAt"
  BEFORE UPDATE ON "sign_ins"
  FOR EACH ROW
  EXECUTE FUNCTION update_updatedAt();