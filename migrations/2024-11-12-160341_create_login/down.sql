-- This file should undo anything in `up.sql`
-- CREATE TABLE "users"(
-- 	"id" TEXT NOT NULL PRIMARY KEY,
-- 	"user_id" TEXT NOT NULL,
-- 	"email" TEXT NOT NULL,
-- 	"password" TEXT NOT NULL,
-- 	"status" TEXT NOT NULL,
-- 	"role" TEXT NOT NULL,
-- 	"salt_value" TEXT NOT NULL
-- );

DROP TABLE IF EXISTS "login_histories";
