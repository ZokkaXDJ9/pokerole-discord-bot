use std::collections::HashMap;
use std::sync::Arc;
use crate::{GameRule, pokemon_api_parser, pokerole_discord_py_csv_parser};

use crate::pokerole_discord_py_csv_parser::{PokeAbility, PokeItem, PokeLearn, PokeMove, PokeStats, PokeStatus, PokeWeather};
use crate::pokemon_api_parser::{PokemonApiData};

/// Data which is stored and accessible in all command invocations
pub struct Data {
    pub abilities: Arc<HashMap<String, PokeAbility>>,
    pub ability_names: Arc<Vec<String>>,
    pub items: Arc<HashMap<String, PokeItem>>,
    pub item_names: Arc<Vec<String>>,
    pub moves: Arc<HashMap<String, PokeMove>>,
    pub move_names: Arc<Vec<String>>,
    pub pokemon: Arc<HashMap<String, PokeStats>>,
    pub pokemon_names: Arc<Vec<String>>,
    pub pokemon_learns: Arc<Vec<PokeLearn>>,
    pub status_effects: Arc<HashMap<String, PokeStatus>>,
    pub status_effects_names: Arc<Vec<String>>,
    pub weather: Arc<HashMap<String, PokeWeather>>,
    pub weather_names: Arc<Vec<String>>,
    pub pokemon_api_data: Arc<HashMap<String, PokemonApiData>>,
    pub rule_names: Arc<Vec<String>>,
    pub rules: Arc<HashMap<String, GameRule>>,
}

pub fn initialize_data() -> Data {
    let pokemon_api_data = pokemon_api_parser::parse_pokemon_api();
    let pokerole_csv_data = pokerole_discord_py_csv_parser::parse("/home/jacudibu/code/pokerole-csv/");

    let raw_rules = vec![
        GameRule {
            name: String::from("Limit Break"),
            flavor: Some(String::from("By investing an extraordinary amount of effort, some pokemon can surpass their natural limits!")),
            text: String::from("You may spend (2 + Amount of previous Limit Breaks) stat points in order to increase your stats past your species' stat cap. For balancing reasons, you can never have more than 10 points in any particular stat, even by using this mechanic."),
            example: Some(String::from("Let's say your max Dexterity is 3. If you want to increase it to 4, you'll need to use two stat points.\n\
                                      Next up, you want to increase your Vitality past its limit. Since you've already used one limit break in the past, this would now cost 3 stat points.\n\
                                      If you stat is already at 10, you cannot limit break it any further."))
        }, GameRule {
            name: String::from("Evolution"),
            flavor: Some(String::from("Sometimes they just grow a little too fast!")),
            text: String::from("You can evolve at any time or even start out as a fully evolved pokemon, but as long as you haven't reached the level required for your evolution yet, you will have to play with the base stats of your unevolved form. Evolution thresholds are Level 3 for second stage, and Level 6 for third stage evolutions. In severe cases where even the second evo has terrible stats, such as for e.g. the Weedle/Kakuna/Beedrill line, you may apply for an exception to be made."),
            example: Some(String::from("Let's say you want to play a Tyranitar. Pupitar is probably not the most fun to play, so you decide to start out fully evolved right from the get go. Until level 3 you will have to play with the base stats of a Larvitar. Once you reach level 3, you can upgrade your base stats to that of a Pupitar, and, finally, once you reach level 6, your base stats may reflect those of a full-powered Tyranitar!"))
        }, GameRule {
            name: String::from("Multi-Target moves"),
            flavor: Some(String::from("\"Watch out, it's the 'Oops, I Did It Everywhere' attack!\"")),
            text: String::from("When using moves targeting *All Foes*, declare the order in which your characters focuses on them. Only the first target can receive a critical hit, and for every successive target hit, reduce the damage die count by 1."),
            example: Some(String::from("You are using Earthquake against three foes! First, you declare the order in which they are to be hit: Ursaring, Absol, Sneasler. Then, you roll accuracy. It's a crit, yay! Roll your regular 8 damage dies against the Ursaring. Critical damage will be applied! Then, roll 7 damage dies against the Lucario. Finally, 6 dies against the Sneasler. Both of those take the regular damage without crit modifiers."))
        }, GameRule {
            name: String::from("Critical Strike"),
            flavor: Some(String::from("Hit 'em right between the legs!")),
            text: String::from("A critical strike occurs when you roll at least three 6 during an accuracy check (You need only 2 for 'High Critical' moves). After the damage reduction from defense, the damage dealt will be increased by 50%, rounded up. If the move applies stat boosts or reductions, those will be increased by 1."),
            example: Some(String::from("You crit and successfully roll 5 damage dies. Your enemy has 2 defense. The final damage dealt is (5 - 2) * 1.5 = 4.5, so 5 damage."))
        }, GameRule {
            name: String::from("Levels and Ranks"),
            flavor: Some(String::from("Now, would you look at this shiny badge?")),
            text: String::from("A level up always requires 100 Experience Points. Experience is gained by going on quests and joining in on events, depending on duration and difficulty/danger levels! For each level up, you gain 1 Stat point, which you may freely spend to increase your stats (see Limit Breaking in case you are maxed out.)\n\
            You'll gain one combat stat point to allocate for each level up!\n\
            \n\
            Furthermore, Levels decide your guild rank and when you can evolve:\n\
            Level 2 => Silver\n\
            Level 3 => Evo #1\n\
            Level 4 => Gold\n\
            Level 6 => Evo #2\n\
            Level 8 => Platinum\n\
            Level 16 => Diamond\n\
            \n\
            Besides the extra die for all the *+ Rank* accuracy rolls, you'll gain 2 social stat points for each rank up!"),
            example: None
        }, GameRule {
            name: String::from("Wound damage"),
            flavor: Some(String::from("It's just a flesh wound!")),
            text: String::from("Wound damage reduces your maximum HP and needs to be healed over time in base and/or by visiting a doctor. If your max HP falls to 0, you lose consciousness and really need some medical assistance!"),
            example: None
        }, GameRule {
            name: String::from("Stat Changes"),
            flavor: Some(String::from("Feeling weak... or... strong? I don't know how to feel.")),
            text: String::from("Stat changes done by different moves stack up to a maximum of +/- 3.\n\
            Critical stat changes apply/reduce one additional stat point, but not past the limit! If it would boost multiple stats, you got to choose one which will receive the crit bonus.\n\
            Defense cannot go below 0, combat stats can't go below 1."),
            example: None
        }, GameRule {
            name: String::from("Alternative Moves (Dodge, Struggle, Help Another)"),
            flavor: Some(String::from("Gotta know the basics if you wanna be an adventurer!")),
            text: String::from("Every Character has access to the following moves at any time:\n\
            - Struggle\n\
            - Help Another\n\
            - Dodge\n"),
            example: None
        }, GameRule {
            name: String::from("Accuracy Reduction"),
            flavor: Some(String::from("Got a little bit of dirt in your eye?")),
            text: String::from("Characters afflicted by accuracy reduction (either as a stat change or through a move effect) still roll the regular amount of accuracy dies, but then subtract -X from the amount of successes this roll had."),
            example: Some(String::from("Let's say you want to use Rock Slide. The move has a -1 Accuracy effect. You roll your regular Dex+Rank amount of dies for your accuracy and land two successes. Now we reduce those successes by -1... which leaves us with one success, so the attack still hits, yay!"))
        }, GameRule {
            name: String::from("Defense"),
            flavor: Some(String::from("How much of a beating can you really take?")),
            text: String::from("You Physical Defense is `Vitality / 2`.\nYour Special Defense is `Insight / 2`.\n\nMinimum damage is always 1, unless you resist the attack's type - this is the only case where it gets reduced to 0."),
            example: Some(String::from("With 4 points in Vitality and 5 points in Insight, you'd get 2 physical and 3 special defense!"))
        }, GameRule {
            name: String::from("Orre Character Import"),
            flavor: Some(String::from("Celebi did another woopsiedoodle!")),
            text: String::from("While everything that happened over in the Orre Region is considered non-canon since this is taking place in a different timeline and in the far future, you can import your characters from Orre over to this system!\n\
            If you where at least Silver Rank, your character may start with Level 2 here.\n\
            If you where Gold or higher, your character may start with Level 3.\n\
            All of these are optional, of course! And no items or other unlocks can be carried over.\n\
            It is fine to start out as an evolved mon here, just check `/rule Evolution` for the details.\n\
            Just follow the character creation guide and apply your two level ups afterwards, those will yield you one Combat Stat Point each and promote you to silver, which also yields two extra Social Stat points!"),
            example: None
        }, GameRule {
            name: String::from("Character Slots"),
            flavor: Some(String::from("Sometimes you just need a friend! Sometimes that friend has to be yourself!")),
            text: String::from("You get one new character slot ever 5 levels (cumulative across all your characters!).\n\
            Using those or creating multiple characters is completely optional, of course.\n\
            Also, if you want more or need one earlier, you may request that from our admin team! We'll vote internally on a case-by-case basis to determine if it would work out."),
            example: None
        }
    ];

    let mut rule_names = Vec::default();
    let mut rule_hash_map = HashMap::default();
    for x in raw_rules {
        rule_names.push(x.name.clone());
        rule_hash_map.insert(x.name.to_lowercase(), x);
    }

    let mut move_names = Vec::default();
    let mut move_hash_map = HashMap::default();
    for x in pokerole_csv_data.moves {
        move_names.push(x.name.clone());
        move_hash_map.insert(x.name.to_lowercase(), x);
    }

    let mut ability_names = Vec::default();
    let mut ability_hash_map = HashMap::default();
    for x in pokerole_csv_data.abilities {
        ability_names.push(x.name.clone());
        ability_hash_map.insert(x.name.to_lowercase(), x);
    }

    let mut weather_names = Vec::default();
    let mut weather_hash_map = HashMap::default();
    for x in pokerole_csv_data.weather {
        weather_names.push(x.name.clone());
        weather_hash_map.insert(x.name.to_lowercase(), x);
    }

    let mut pokemon_names = Vec::default();
    let mut pokemon = HashMap::default();
    for x in pokerole_csv_data.stats {
        if x.name.starts_with("Delta ") {
            continue;
        }

        pokemon_names.push(x.name.clone());
        pokemon.insert(x.name.to_lowercase(), x);
    }

    let mut status_names = Vec::default();
    let mut status_hash_map = HashMap::default();
    for x in pokerole_csv_data.status_effects {
        status_names.push(x.name.clone());
        status_hash_map.insert(x.name.to_lowercase(), x);
    }

    let mut item_names = Vec::default();
    let mut item_hash_map = HashMap::default();
    for x in pokerole_csv_data.items {
        if x.description.is_empty() {
            continue;
        }

        item_names.push(x.name.clone());
        item_hash_map.insert(x.name.to_lowercase(), x);
    }

    Data {
        abilities: Arc::new(ability_hash_map),
        ability_names: Arc::new(ability_names),
        items: Arc::new(item_hash_map),
        item_names: Arc::new(item_names),
        moves: Arc::new(move_hash_map),
        move_names: Arc::new(move_names),
        pokemon: Arc::new(pokemon),
        pokemon_names: Arc::new(pokemon_names),
        pokemon_learns: Arc::new(pokerole_csv_data.learns),
        rules: Arc::new(rule_hash_map),
        rule_names: Arc::new(rule_names),
        status_effects: Arc::new(status_hash_map),
        status_effects_names: Arc::new(status_names),
        weather: Arc::new(weather_hash_map),
        weather_names: Arc::new(weather_names),
        pokemon_api_data: Arc::new(pokemon_api_data),
    }
}
