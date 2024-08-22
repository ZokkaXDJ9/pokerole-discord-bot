use crate::{Error, commands::Context};
use rand::Rng;

#[derive(Clone)]
struct ItemWithRarity {
    name: String,
    rarity: u16, // Represents a relative chance out of 1000
}

#[poise::command(slash_command)]
pub async fn open_box(ctx: Context<'_>) -> Result<(), Error> {
    let item = get_box_item();

    match item {
        Some(item) => {
            ctx.say(format!("You opened a Treasure Box and received: {}", item)).await?;
        }
        None => {
            ctx.say("Something went wrong, no item was found in the Treasure Box.").await?;
        }
    }
    Ok(())
}

fn get_box_item() -> Option<String> {
    let items = vec![
        // Berries (400 in 1000 total chance)
        ItemWithRarity { name: "Oran Berry".to_string(), rarity: 30 },
        ItemWithRarity { name: "Cherri Berry".to_string(), rarity: 30 },
        ItemWithRarity { name: "Chesto Berry".to_string(), rarity: 30 },
        ItemWithRarity { name: "Rawst Berry".to_string(), rarity: 30 },
        ItemWithRarity { name: "Pecha Berry".to_string(), rarity: 30 },
        ItemWithRarity { name: "Aspear Berry".to_string(), rarity: 30 },
        ItemWithRarity { name: "Persim Berry".to_string(), rarity: 30 },
        ItemWithRarity { name: "Eggant Berry".to_string(), rarity: 30 },
        // Uncommon Berries
        ItemWithRarity { name: "Sitrus Berry".to_string(), rarity: 15 },
        ItemWithRarity { name: "Lum Berry".to_string(), rarity: 15 },
        ItemWithRarity { name: "Leichi Berry".to_string(), rarity: 15 },
        ItemWithRarity { name: "Ganlon Berry".to_string(), rarity: 15 },
        ItemWithRarity { name: "Petaya Berry".to_string(), rarity: 15 },
        ItemWithRarity { name: "Apicot Berry".to_string(), rarity: 15 },
        ItemWithRarity { name: "Salac Berry".to_string(), rarity: 15 },
        ItemWithRarity { name: "Lansat Berry".to_string(), rarity: 15 },
        ItemWithRarity { name: "Starf Berry".to_string(), rarity: 15 },
        // Rare Berries
        ItemWithRarity { name: "Leppa Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Occa Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Passho Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Wacan Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Rindo Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Yache Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Chople Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Kebia Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Shuca Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Coba Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Payapa Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Tanga Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Charti Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Kasib Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Haban Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Colbur Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Babiri Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Chilan Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Roseli Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Pumkin Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Drash Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Bitmel Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Chipe Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Nomel Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Nutpea Berry".to_string(), rarity: 4 },
        ItemWithRarity { name: "Magost Berry".to_string(), rarity: 4 },
        // Very Rare Berry
        ItemWithRarity { name: "Enigma Berry".to_string(), rarity: 2 }, // Adjusted to be less common than rare berries

        // Seeds (250 in 1000 total chance)
        ItemWithRarity { name: "Blast Seed".to_string(), rarity: 75 },
        ItemWithRarity { name: "Stun Seed".to_string(), rarity: 75 },
        ItemWithRarity { name: "Sleep Seed".to_string(), rarity: 75 },
        // Uncommon Seeds
        ItemWithRarity { name: "Encourage Seed".to_string(), rarity: 15 }, // Less common than common seeds
        // Rare Seeds
        ItemWithRarity { name: "Reviver Seed".to_string(), rarity: 10 }, // The rarest seed

        // TMs (200 in 1000 total chance)
        ItemWithRarity { name: "TM Metronome".to_string(), rarity: 120 },
        ItemWithRarity { name: "TM Status Move".to_string(), rarity: 50 },
        ItemWithRarity { name: "TM Power 2-3 Move".to_string(), rarity: 20 },
        ItemWithRarity { name: "TM Power 5+ Move".to_string(), rarity: 10 },

        // Held Items (150 in 1000 total chance)
        // Common Held Items (4 points each)
        ItemWithRarity { name: "Normal Gem".to_string(), rarity: 4 },
        ItemWithRarity { name: "Fire Gem".to_string(), rarity: 4 },
        ItemWithRarity { name: "Water Gem".to_string(), rarity: 4 },
        ItemWithRarity { name: "Electric Gem".to_string(), rarity: 4 },
        ItemWithRarity { name: "Grass Gem".to_string(), rarity: 4 },
        ItemWithRarity { name: "Ice Gem".to_string(), rarity: 4 },
        ItemWithRarity { name: "Fighting Gem".to_string(), rarity: 4 },
        ItemWithRarity { name: "Poison Gem".to_string(), rarity: 4 },
        ItemWithRarity { name: "Ground Gem".to_string(), rarity: 4 },
        ItemWithRarity { name: "Flying Gem".to_string(), rarity: 4 },
        ItemWithRarity { name: "Psychic Gem".to_string(), rarity: 4 },
        ItemWithRarity { name: "Bug Gem".to_string(), rarity: 4 },
        ItemWithRarity { name: "Rock Gem".to_string(), rarity: 4 },
        ItemWithRarity { name: "Ghost Gem".to_string(), rarity: 4 },
        ItemWithRarity { name: "Dragon Gem".to_string(), rarity: 4 },
        ItemWithRarity { name: "Dark Gem".to_string(), rarity: 4 },
        ItemWithRarity { name: "Steel Gem".to_string(), rarity: 4 },
        ItemWithRarity { name: "Fairy Gem".to_string(), rarity: 4 },
        ItemWithRarity { name: "Air Balloon".to_string(), rarity: 4 },
        ItemWithRarity { name: "Absorb Bulb".to_string(), rarity: 4 },
        ItemWithRarity { name: "Luminous Moss".to_string(), rarity: 4 },
        ItemWithRarity { name: "Cell Battery".to_string(), rarity: 4 },
        ItemWithRarity { name: "Snowball".to_string(), rarity: 4 },
        ItemWithRarity { name: "Choice Band".to_string(), rarity: 4 },
        ItemWithRarity { name: "Choice Scarf".to_string(), rarity: 4 },
        ItemWithRarity { name: "Choice Specs".to_string(), rarity: 4 },
        ItemWithRarity { name: "Throat Spray".to_string(), rarity: 4 },

        // Uncommon Held Items (3 points each)
        ItemWithRarity { name: "Focus Sash".to_string(), rarity: 3 },
        ItemWithRarity { name: "Black Belt".to_string(), rarity: 3 },
        ItemWithRarity { name: "Black Glasses".to_string(), rarity: 3 },
        ItemWithRarity { name: "Charcoal".to_string(), rarity: 3 },
        ItemWithRarity { name: "Dragon Fang".to_string(), rarity: 3 },
        ItemWithRarity { name: "Fairy Feather".to_string(), rarity: 3 },
        ItemWithRarity { name: "Hard Stone".to_string(), rarity: 3 },
        ItemWithRarity { name: "Magnet".to_string(), rarity: 3 },
        ItemWithRarity { name: "Metal Coat".to_string(), rarity: 3 },
        ItemWithRarity { name: "Miracle Seed".to_string(), rarity: 3 },
        ItemWithRarity { name: "Mystic Water".to_string(), rarity: 3 },
        ItemWithRarity { name: "Never-Melt Ice".to_string(), rarity: 3 },
        ItemWithRarity { name: "Poison Barb".to_string(), rarity: 3 },
        ItemWithRarity { name: "Sharp Beak".to_string(), rarity: 3 },
        ItemWithRarity { name: "Silk Scarf".to_string(), rarity: 3 },
        ItemWithRarity { name: "Silver Powder".to_string(), rarity: 3 },
        ItemWithRarity { name: "Soft Sand".to_string(), rarity: 3 },
        ItemWithRarity { name: "Spell Tag".to_string(), rarity: 3 },
        ItemWithRarity { name: "Twisted Spoon".to_string(), rarity: 3 },
        ItemWithRarity { name: "Leftovers".to_string(), rarity: 3 },
        ItemWithRarity { name: "Black Sludge".to_string(), rarity: 3 },
        ItemWithRarity { name: "Sticky Barbs".to_string(), rarity: 3 },
        ItemWithRarity { name: "Destiny Knot".to_string(), rarity: 3 },
        ItemWithRarity { name: "Damp Rock".to_string(), rarity: 3 },
        ItemWithRarity { name: "Icy Rock".to_string(), rarity: 3 },
        ItemWithRarity { name: "Heat Rock".to_string(), rarity: 3 },
        ItemWithRarity { name: "Smooth Rock".to_string(), rarity: 3 },
        ItemWithRarity { name: "Terrain Extender".to_string(), rarity: 3 },
        ItemWithRarity { name: "Electric Seed".to_string(), rarity: 3 },
        ItemWithRarity { name: "Grassy Seed".to_string(), rarity: 3 },
        ItemWithRarity { name: "Misty Seed".to_string(), rarity: 3 },
        ItemWithRarity { name: "Psychic Seed".to_string(), rarity: 3 },
        ItemWithRarity { name: "Room Service".to_string(), rarity: 3 },
        ItemWithRarity { name: "Mental Herb".to_string(), rarity: 3 },
        ItemWithRarity { name: "Power Herb".to_string(), rarity: 3 },
        ItemWithRarity { name: "Flame Orb".to_string(), rarity: 3 },
        ItemWithRarity { name: "Float Stone".to_string(), rarity: 3 },
        ItemWithRarity { name: "Iron Ball".to_string(), rarity: 3 },
        ItemWithRarity { name: "Lagging Tail".to_string(), rarity: 3 },
        ItemWithRarity { name: "Big Root".to_string(), rarity: 3 },
        ItemWithRarity { name: "Ability Shield".to_string(), rarity: 3 },
        ItemWithRarity { name: "Mirror Herb".to_string(), rarity: 3 },

        // Rare Held Items (2 points each)
        ItemWithRarity { name: "King's Rock".to_string(), rarity: 2 },
        ItemWithRarity { name: "Quick Claw".to_string(), rarity: 2 },
        ItemWithRarity { name: "Eviolite".to_string(), rarity: 2 },
        ItemWithRarity { name: "Rocky Helmet".to_string(), rarity: 2 },
        ItemWithRarity { name: "Expert Belt".to_string(), rarity: 2 },
        ItemWithRarity { name: "Weakness Policy".to_string(), rarity: 2 },
        ItemWithRarity { name: "Bright Powder".to_string(), rarity: 2 },
        ItemWithRarity { name: "Grip Claw".to_string(), rarity: 2 },
        ItemWithRarity { name: "Light Clay".to_string(), rarity: 2 },
        ItemWithRarity { name: "Muscle Band".to_string(), rarity: 2 },

        // Very Rare Held Items (1 point each)
        ItemWithRarity { name: "Wide Lens".to_string(), rarity: 1 },
        ItemWithRarity { name: "Life Orb".to_string(), rarity: 1 },
        ItemWithRarity { name: "Razor Claw".to_string(), rarity: 1 },
        ItemWithRarity { name: "Scope Lens".to_string(), rarity: 1 },
        ItemWithRarity { name: "Shell Bell".to_string(), rarity: 1 },
        ItemWithRarity { name: "Safety Goggles".to_string(), rarity: 1 },
        ItemWithRarity { name: "Blunder Policy".to_string(), rarity: 1 },
        ItemWithRarity { name: "Assault Vest".to_string(), rarity: 1 },
        ItemWithRarity { name: "Binding Band".to_string(), rarity: 1 },
        ItemWithRarity { name: "Metronome".to_string(), rarity: 1 },
        ItemWithRarity { name: "Clear Amulet".to_string(), rarity: 1 },
        ItemWithRarity { name: "Covert Cloak".to_string(), rarity: 1 },
        ItemWithRarity { name: "Loaded Dice".to_string(), rarity: 1 },
    ];

    let total_rarity: u16 = items.iter().map(|item| item.rarity).sum();
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(0..total_rarity);

    let mut cumulative_rarity = 0;
    for item in items {
        cumulative_rarity += item.rarity;
        if roll < cumulative_rarity {
            return Some(item.name);
        }
    }
    None
}
