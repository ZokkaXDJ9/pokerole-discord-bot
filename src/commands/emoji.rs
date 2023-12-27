use crate::commands::autocompletion::autocomplete_pokemon;
use crate::commands::{send_ephemeral_reply, Context, Error};
use crate::enums::{Gender, PokemonGeneration, RegionalVariant};
use crate::game_data::pokemon::Pokemon;
use image::{DynamicImage, GenericImageView, ImageOutputFormat};
use log::info;
use poise::CreateReply;
use serenity::all::{CreateAttachment, Emoji};
use std::io::{BufReader, Cursor, Read, Seek};

const GEN5_ANIMATED: &str = "https://github.com/PokeAPI/sprites/blob/master/sprites/pokemon/versions/generation-v/black-white/animated/";
const GEN5_ANIMATED_FEMALE: &str = "https://github.com/PokeAPI/sprites/blob/master/sprites/pokemon/versions/generation-v/black-white/animated/female/";

const FRONT_MALE: &str = "https://github.com/PokeAPI/sprites/blob/master/sprites/pokemon/";
const FRONT_FEMALE: &str = "https://github.com/PokeAPI/sprites/blob/master/sprites/pokemon/female/";

fn build_string(pokemon: &Pokemon) -> String {
    let mut result = String::from("");

    if pokemon.species_data.generation <= PokemonGeneration::Five {
        result.push_str(
            std::format!(
                "\
## Animated Gen 5 sprite
Male/Unisex: <{}{}.gif>\n",
                GEN5_ANIMATED,
                pokemon.poke_api_id.0
            )
            .as_str(),
        );

        if pokemon.species_data.has_gender_differences {
            result.push_str(
                std::format!(
                    "Female: <{}{}.gif>\n",
                    GEN5_ANIMATED_FEMALE,
                    pokemon.poke_api_id.0
                )
                .as_str(),
            );
        }
    }

    result.push_str(
        std::format!(
            "\
## Regular Front Sprite
Male/Unisex: <{}{}.png>\n",
            FRONT_MALE,
            pokemon.poke_api_id.0
        )
        .as_str(),
    );

    if pokemon.species_data.has_gender_differences {
        result.push_str(
            std::format!("Female: <{}{}.png>\n", FRONT_FEMALE, pokemon.poke_api_id.0).as_str(),
        );
    }

    result.push_str("\n\n**When adding the emoji to the server, make sure to cut out the whitespace around the sprite, and make it square sized so discord doesn't stretch it in some awkward way.**");

    result
}

const EMPTY_PIXEL: [u8; 4] = [0, 0, 0, 0];

fn find_top_border(image: &DynamicImage) -> u32 {
    for y in 0..image.height() {
        let mut contains_something = false;
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            if pixel.0 != EMPTY_PIXEL {
                contains_something = true;
                break;
            }
        }

        if contains_something {
            return y;
        }
    }

    0
}

fn find_bottom_border(image: &DynamicImage) -> u32 {
    for y in (0..image.height()).rev() {
        let mut contains_something = false;
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            if pixel.0 != EMPTY_PIXEL {
                contains_something = true;
                break;
            }
        }

        if contains_something {
            return y + 1;
        }
    }

    image.height()
}

fn find_left_border(image: &DynamicImage) -> u32 {
    for x in 0..image.width() {
        let mut contains_something = false;
        for y in 0..image.height() {
            let pixel = image.get_pixel(x, y);
            if pixel.0 != EMPTY_PIXEL {
                contains_something = true;
                break;
            }
        }

        if contains_something {
            return x;
        }
    }

    0
}

fn find_right_border(image: &DynamicImage) -> u32 {
    for x in (0..image.width()).rev() {
        let mut contains_something = false;
        for y in 0..image.height() {
            let pixel = image.get_pixel(x, y);
            if pixel.0 != EMPTY_PIXEL {
                contains_something = true;
                break;
            }
        }

        if contains_something {
            return x + 1;
        }
    }

    image.width()
}

struct EmojiData {
    data: Vec<u8>,
    name: String,
}

fn local_emoji_path(
    pokemon: &Pokemon,
    is_female: bool,
    is_shiny: bool,
    is_animated: bool,
) -> String {
    let path = std::env::var("POKEMON_API_SPRITES")
        .expect("missing POKEMON_API_SPRITES environment variable.");

    let animated_path = if is_animated {
        "versions/generation-v/black-white/animated/"
    } else {
        ""
    };
    let shiny_path = if is_shiny { "shiny/" } else { "" };
    let gender_path = if is_female { "female/" } else { "" };

    format!(
        "{}{}/sprites/pokemon/{}{}{}.png",
        path, animated_path, shiny_path, gender_path, pokemon.poke_api_id.0
    )
}

fn get_emoji_data(
    pokemon: &Pokemon,
    gender: &Gender,
    is_shiny: bool,
    is_animated: bool,
) -> Result<EmojiData, Error> {
    let use_female_sprite =
        pokemon.species_data.has_gender_differences && gender == &Gender::Female;

    let path = local_emoji_path(pokemon, use_female_sprite, is_shiny, is_animated);
    let image = image::open(path)?;

    let image = crop_whitespace(image);
    let mut cursor = Cursor::new(Vec::new());
    image.write_to(&mut cursor, ImageOutputFormat::Png)?;

    cursor.rewind()?;
    let reader = &mut BufReader::new(&mut cursor);
    let mut out = Vec::new();
    reader.read_to_end(&mut out)?;

    let shiny = if is_shiny { "shiny_" } else { "" };
    let female = if use_female_sprite { "_female" } else { "" };
    let mut name = pokemon
        .name
        .to_lowercase()
        .replace(" ", "_")
        .replace("(", "")
        .replace(")", "");

    let regional_prefix = if let Some(regional_variant) = pokemon.regional_variant {
        name = name
            .replace("paldean_form", "")
            .replace("hisuian_form", "")
            .replace("galarian_form", "")
            .replace("alolan_form", "");

        match regional_variant {
            RegionalVariant::Alola => "alolan_",
            RegionalVariant::Galar => "galarian",
            RegionalVariant::Hisui => "hisuian_",
            RegionalVariant::Paldea => "paldean_",
        }
    } else {
        ""
    };

    let name = name.trim_matches('_');

    Ok(EmojiData {
        data: out,
        name: format!("{}{}{}{}", shiny, regional_prefix, name, female),
    })
}

fn crop_whitespace(image: DynamicImage) -> DynamicImage {
    let top_border = find_top_border(&image);
    let bottom_border = find_bottom_border(&image);
    let left_border = find_left_border(&image);
    let right_border = find_right_border(&image);

    image.crop_imm(
        left_border,
        top_border,
        right_border - left_border,
        bottom_border - top_border,
    )
}

async fn upload_emoji_to_discord<'a>(
    ctx: &Context<'a>,
    emoji_data: EmojiData,
) -> Result<Emoji, String> {
    let guild_id = ctx.guild_id().unwrap();
    let attachment = CreateAttachment::bytes(emoji_data.data, &emoji_data.name);
    match guild_id
        .create_emoji(&ctx, emoji_data.name.as_str(), &attachment.to_base64())
        .await
    {
        Ok(emoji) => {
            let _ = send_ephemeral_reply(ctx, &format!("Created new emoji: {}", emoji)).await;
            Ok(emoji)
        }
        Err(e) => Err(e.to_string()),
    }
}

/// Display links to fancy emojis!
#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
pub async fn emoji(
    ctx: Context<'_>,
    #[description = "Which pokemon?"]
    #[rename = "pokemon"]
    #[autocomplete = "autocomplete_pokemon"]
    name: String,
    #[description = "Which phenotype?"] gender: Gender,
    #[description = "Does it glow in the dark?"] is_shiny: bool,
) -> Result<(), Error> {
    if let Some(pokemon) = ctx.data().game.pokemon.get(&name.to_lowercase()) {
        if let Ok(emoji_data) = get_emoji_data(pokemon, &gender, is_shiny, false) {
            let _ = upload_emoji_to_discord(&ctx, emoji_data).await;
        }

        if pokemon.species_data.generation <= PokemonGeneration::Five {
            if let Ok(emoji_data) = get_emoji_data(pokemon, &gender, is_shiny, true) {
                let _ = upload_emoji_to_discord(&ctx, emoji_data).await;
            }
        }
    } else {
        ctx.send(CreateReply::default()
            .content(std::format!("Unable to find a pokemon named **{}**, sorry! If that wasn't a typo, maybe it isn't implemented yet?", name))
            .ephemeral(true)
        ).await?;
    }

    Ok(())
}
