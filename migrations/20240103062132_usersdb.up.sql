-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE IF NOT EXISTS usersdb (
    "id" uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    "username" varchar NOT NULL,
	"password" varchar NOT NULL,
    "email" varchar NOT NULL,
    "date_created" TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    "date_updated" TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);