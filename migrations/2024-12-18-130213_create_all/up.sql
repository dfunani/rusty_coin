-- Your SQL goes here

CREATE TABLE "profiles"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"profile_id" TEXT NOT NULL,
	"account_id" TEXT NOT NULL,
	"first_name" TEXT NOT NULL,
	"last_name" TEXT NOT NULL,
	"username" TEXT NOT NULL,
	"date_of_birth" DATE NOT NULL,
	"gender" TEXT NOT NULL,
	"profile_picture" TEXT NOT NULL,
	"mobile_number" TEXT NOT NULL,
	"country" TEXT NOT NULL,
	"language" TEXT NOT NULL,
	"biography" TEXT NOT NULL,
	"occupation" TEXT NOT NULL,
	"interests" TEXT NOT NULL,
	"social_media_links" TEXT NOT NULL,
	"status" TEXT NOT NULL,
	"created_date" TIMESTAMP NOT NULL,
	"updated_date" TIMESTAMP NOT NULL
);

CREATE TABLE "payments"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"payment_id" TEXT NOT NULL,
	"account_id" TEXT NOT NULL,
	"card_id" TEXT NOT NULL,
	"name" TEXT NOT NULL,
	"description" TEXT NOT NULL,
	"status" TEXT NOT NULL,
	"balance" FLOAT NOT NULL,
	"created_date" TIMESTAMP NOT NULL,
	"updated_date" TIMESTAMP NOT NULL
);

CREATE TABLE "accounts"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"account_id" TEXT NOT NULL,
	"user_id" TEXT NOT NULL,
	"status" TEXT NOT NULL,
	"created_date" TIMESTAMP NOT NULL,
	"updated_date" TIMESTAMP NOT NULL
);

CREATE TABLE "settings"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"settings_id" TEXT NOT NULL,
	"account_id" TEXT NOT NULL,
	"email_status" TEXT NOT NULL,
	"communication_status" TEXT NOT NULL,
	"mfa_enabled" TEXT NOT NULL,
	"mfa_last_used_date" TIMESTAMP NOT NULL,
	"profile_visibility_preference" TEXT NOT NULL,
	"data_sharing_preferences" TEXT NOT NULL,
	"communication_preference" TEXT NOT NULL,
	"location_tracking_enabled" BOOL NOT NULL,
	"cookies_enabled" BOOL NOT NULL,
	"theme_preference" TEXT NOT NULL,
	"created_date" TIMESTAMP NOT NULL,
	"updated_date" TIMESTAMP NOT NULL
);

