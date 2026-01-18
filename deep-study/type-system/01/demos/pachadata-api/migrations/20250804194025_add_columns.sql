ALTER TABLE courses 
ADD COLUMN course_type TEXT NOT NULL DEFAULT 'Online';

ALTER TABLE courses
ADD COLUMN url TEXT NULL;

ALTER TABLE courses
ADD COLUMN location TEXT NULL;

ALTER TABLE courses
ADD COLUMN video_link TEXT NULL;