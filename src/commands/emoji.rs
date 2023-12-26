use crate::commands::autocompletion::autocomplete_pokemon;
use crate::commands::{Context, Error};
use crate::enums::PokemonGeneration;
use crate::game_data::pokemon::Pokemon;
use image::{DynamicImage, GenericImageView, ImageOutputFormat};
use log::info;
use poise::CreateReply;
use serenity::all::CreateAttachment;
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

fn crop_whitespace() -> DynamicImage {
    let path = std::env::var("POKEMON_API_SPRITES")
        .expect("missing POKEMON_API_SPRITES environment variable.");
    let image = image::open(path + "/sprites/pokemon/25.png").unwrap();

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

async fn create_emoji<'a>(ctx: &Context<'a>, image: Vec<u8>) {
    let guild_id = ctx.guild_id().unwrap();
    let attachment = CreateAttachment::bytes(image, "pikachu.png");
    match guild_id
        .create_emoji(&ctx, "pikachu", &attachment.to_base64())
        .await
    {
        Ok(emoji) => {
            let _ = ctx.say(format!("Created new emoji: {}", emoji)).await;
        }
        Err(e) => {
            info!("{}", e);
        }
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
) -> Result<(), Error> {
    if let Some(pokemon) = ctx.data().game.pokemon.get(&name.to_lowercase()) {
        let image = crop_whitespace();
        let mut cursor = Cursor::new(Vec::new());
        let result = image.write_to(&mut cursor, ImageOutputFormat::Png);

        let _ = cursor.rewind();
        let reader = &mut BufReader::new(&mut cursor);
        let mut out = Vec::new();
        let result = reader.read_to_end(&mut out);

        create_emoji(&ctx, out).await;

        ctx.send(CreateReply::default().content(build_string(pokemon)))
            .await?;
    } else {
        ctx.send(CreateReply::default()
            .content(std::format!("Unable to find a pokemon named **{}**, sorry! If that wasn't a typo, maybe it isn't implemented yet?", name))
            .ephemeral(true)
        ).await?;
    }

    Ok(())
}
