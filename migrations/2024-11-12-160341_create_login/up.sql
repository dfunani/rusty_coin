-- Your SQL goes here
-- DROP TABLE IF EXISTS "users";
CREATE TABLE "login_histories"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"login_id" TEXT NOT NULL,
	"user_id" TEXT NOT NULL,
	"session_id" TEXT NOT NULL,
	"login_date" TIMESTAMP NOT NULL,
	"login_location" TEXT NOT NULL,
	"login_device" TEXT NOT NULL,
	"login_method" TEXT NOT NULL,
	"logged_in" BOOLEAN NOT NULL,
	"logout_date" TIMESTAMP NOT NULL,
	"authentication_token" TEXT NOT NULL,
	"created_date" TIMESTAMP NOT NULL,
	"updated_date" TIMESTAMP NOT NULL
);

