A discord bot to support playing a customized, Mystery-Dungeon Themed version of [Pokerole](https://www.pokeroleproject.com).

Since this is my first "bigger" rust project, and I'm still in the process of learning the language, things might not look very nice at times. This whole thing will be a never-ending, ongoing refactoring process. :D

You can see the bot in action over on the [development discord server](https://discord.gg/jVrv2YG2zU). If you are fine with our custom changes to the Pokerole Systems, feel free to just add it to your own server and use it there. Any persistent data is server specific, so there's no need to worry about breaking anything.

## Environment Variables
We poll and combine data from different sources, this allows us to update the data inside the bot without requiring a new build for every minor addition. In order to get this to work, the executable requires the following environment variables to be set:
- **POKEMON_API** – path to a local clone of the [pokeAPI](https://github.com/PokeAPI/pokeapi) git repository
- **POKEROLE_DATA** – path to a local clone of the [Pokerole-Data](https://github.com/Pokerole-Software-Development/Pokerole-Data) git repository
- **CUSTOM_DATA** – path to your custom data overrides. In our case, that's https://github.com/Jacudibu/pokerole-custom-data.
- **DISCORD_TOKEN** – the Discord Token for your bot.
- **DATABASE_URL** – URL to the SQLite database file.
- **DB_BACKUP_CHANNEL_ID** – Optional. Discord Channel ID into which daily backups should be posted.

## Contributing
Contributions of any kind are always welcome!
If it's a bigger feature just let me know what you want to work on by creating an issue first, to avoid accidental duplicate work.
Also feel free to join the [development discord server](https://discord.gg/jVrv2YG2zU)!

## TODO
A small list of ideas which might be useful to explore in the future.
- Character Creator: Support the character creation process by adding a step-by-step guide accessible through the bot.
- Weather: Random Weather changes which get posted into a specific channel whenever nothing has happened for a while.
- Move usage tracking: What's a user's most used move?
- Guild Wallets: Automatically add guild taxes to a guild wallet on quest completion.
