-- Add migration script here
CREATE TABLE tickets (
	id	BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
	title	TEXT NOT NULL,
	PRIMARY KEY (id)
);