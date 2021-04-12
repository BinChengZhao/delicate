-- Your SQL goes here
CREATE TABLE posts (
  `id` bigint PRIMARY KEY,
  `title` varchar(255) NOT NULL,
  `body` TEXT NOT NULL,
  `published` smallint NOT NULL DEFAULT '0'
)