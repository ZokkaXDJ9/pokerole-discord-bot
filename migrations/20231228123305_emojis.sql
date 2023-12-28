CREATE TABLE emoji(
                     species_api_id INTEGER NOT NULL,
                     guild_id INTEGER NOT NULL,
                     is_female boolean NOT NULL,
                     is_shiny boolean NOT NULL,
                     is_animated boolean NOT NULL,
                     discord_string TEXT NOT NULL,
                     FOREIGN KEY (guild_id) REFERENCES guild(id),
                     UNIQUE(species_api_id, guild_id, is_female, is_shiny, is_animated)
);
