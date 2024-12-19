-- Your SQL goes here
CREATE TABLE "users"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"user_id" TEXT NOT NULL,
	"email" TEXT NOT NULL,
	"password" TEXT NOT NULL,
	"status" TEXT NOT NULL,
	"role" TEXT NOT NULL,
	"salt_value" TEXT NOT NULL,
	"created_date" TIMESTAMP NOT NULL,
	"updated_date" TIMESTAMP NOT NULL
);

