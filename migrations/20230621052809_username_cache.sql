ALTER TABLE user DROP COLUMN mention;

ALTER TABLE guild DROP COLUMN money;
ALTER TABLE guild DROP COLUMN action_log_channel_id;

ALTER TABLE guild ADD COLUMN money INTEGER NOT NULL DEFAULT 0;
ALTER TABLE guild ADD COLUMN action_log_channel_id INTEGER;

CREATE TABLE user_in_guild(
    user_id INTEGER NOT NULL,
    guild_id INTEGER NOT NULL,
    name TEXT NOT NULL COLLATE NOCASE,
    PRIMARY KEY (user_id, guild_id),
    FOREIGN KEY (user_id) REFERENCES user(id),
    FOREIGN KEY (guild_id) REFERENCES guild(id)
);
