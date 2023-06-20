CREATE TABLE user(
    id INTEGER NOT NULL PRIMARY KEY,
    mention TEXT NOT NULL COLLATE NOCASE,
    setting_time_offset_hours INTEGER,
    setting_time_offset_minutes INTEGER
);
