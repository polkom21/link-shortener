-- Your SQL goes here
CREATE TABLE links (
	id serial NOT NULL PRIMARY KEY,
	short varchar(64) NOT NULL,
	original text NOT NULL,
	created timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);