-- Add total_modules column to existing courses table
ALTER TABLE courses 
ADD COLUMN total_modules INTEGER NOT NULL DEFAULT 0;

ALTER TABLE courses 
ADD COLUMN total_minutes INTEGER NOT NULL DEFAULT 0;


-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL
);

-- Create course_progress table
CREATE TABLE IF NOT EXISTS course_progress (
    user_id TEXT NOT NULL,
    course_id TEXT NOT NULL,
    completed_modules INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, course_id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Create webinar_progress table
CREATE TABLE IF NOT EXISTS webinar_progress (
    user_id TEXT NOT NULL,
    webinar_id TEXT NOT NULL,
    minutes_watched INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, webinar_id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Seed demo user
INSERT OR IGNORE INTO users (id, name, email, created_at) VALUES
('550e8400-e29b-41d4-a716-446655440000', 'Alice Example', 'alice@example.com', datetime('now'));

-- Seed course progress
INSERT OR REPLACE INTO course_progress (user_id, course_id, completed_modules) VALUES
('550e8400-e29b-41d4-a716-446655440000', 'c1', 6);

-- Seed webinar progress
INSERT OR REPLACE INTO webinar_progress (user_id, webinar_id, minutes_watched) VALUES
('550e8400-e29b-41d4-a716-446655440000', 'w1', 45);
