-- Add migration script here
CREATE TABLE post (
    id VARCHAR NOT NULL PRIMARY KEY,
    dt VARCHAR NOT NULL,
    image_url VARCHAR,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
