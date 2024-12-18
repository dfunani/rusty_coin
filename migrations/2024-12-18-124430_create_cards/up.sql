-- DROP TABLE IF EXISTS "users";

CREATE TABLE "cards"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"card_id" TEXT NOT NULL,
	"card_number" TEXT NOT NULL,
	"card_type" TEXT NOT NULL,
	"status" TEXT NOT NULL,
	"pin" TEXT NOT NULL,
	"salt_value" TEXT NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL,
);
