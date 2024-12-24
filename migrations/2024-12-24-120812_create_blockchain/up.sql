-- Your SQL goes here

CREATE TABLE "blocks"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"block_id" TEXT NOT NULL,
	"transaction_id" TEXT NOT NULL,
	"contract_id" TEXT NOT NULL,
	"previous_block_id" TEXT NOT NULL,
	"next_block_id" TEXT NOT NULL,
	"block_type" TEXT NOT NULL,
	"created_date" TIMESTAMP NOT NULL,
	"updated_date" TIMESTAMP NOT NULL
);

CREATE TABLE "contracts"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"contract_id" TEXT NOT NULL,
	"contractor" TEXT NOT NULL,
	"contractee" TEXT NOT NULL,
	"title" TEXT NOT NULL,
	"description" TEXT NOT NULL,
	"contract" TEXT NOT NULL,
	"contract_status" TEXT NOT NULL,
	"contractor_signiture" TEXT NOT NULL,
	"contractee_signiture" TEXT NOT NULL,
	"salt_value" TEXT NOT NULL,
	"created_date" TIMESTAMP NOT NULL,
	"updated_date" TIMESTAMP NOT NULL
);

CREATE TABLE "transactions"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"transaction_id" TEXT NOT NULL,
	"sender" TEXT NOT NULL,
	"receiver" TEXT NOT NULL,
	"amount" FLOAT NOT NULL,
	"title" TEXT NOT NULL,
	"description" TEXT NOT NULL,
	"sender_signiture" TEXT NOT NULL,
	"receiver_signiture" TEXT NOT NULL,
	"transaction_status" TEXT NOT NULL,
	"salt_value" TEXT NOT NULL,
	"created_date" TIMESTAMP NOT NULL,
	"updated_date" TIMESTAMP NOT NULL
);

