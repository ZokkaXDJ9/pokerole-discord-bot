ALTER TABLE user
DROP COLUMN mention;

CREATE TABLE user_in_guild(
    user_id INTEGER NOT NULL,
    guild_id INTEGER NOT NULL,
    name TEXT NOT NULL COLLATE NOCASE,
    PRIMARY KEY (user_id, guild_id),
    FOREIGN KEY (user_id) REFERENCES user(id),
    FOREIGN KEY (guild_id) REFERENCES guild(id)
);
