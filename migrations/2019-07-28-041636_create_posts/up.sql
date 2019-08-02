-- Your SQL goes here
CREATE TABLE posts (
       id INTEGER PRIMARY KEY ASC NOT NULL,
       link_id TEXT NOT NULL UNIQUE,
       video_link TEXT NOT NULL,
       timestamp INTEGER NOT NULL
)
