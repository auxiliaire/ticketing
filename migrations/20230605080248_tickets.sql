-- Add migration script here
CREATE TABLE "tickets" (
	"id"	INTEGER NOT NULL UNIQUE,
	"title"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);