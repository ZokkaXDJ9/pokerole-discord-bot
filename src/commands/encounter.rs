use rand::{Rng, thread_rng};
use rand::seq::IteratorRandom;
use crate::commands::{Context, Error};
use crate::commands::autocompletion::autocomplete_pokemon;
use crate::data::pokemon::{Pokemon};
use crate::enums::MysteryDungeonRank;


/// Encounter some wild pokemon!
#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
pub async fn encounter(
    ctx: Context<'_>,
    #[description = "Which pokemon?"]
    #[autocomplete = "autocomplete_pokemon"]
    pokemon: String,
    #[min = 1_u8]
    #[description = "Of which level?"]
    level: u8,
    #[min = 1_u8]
    #[max = 5_u8]
    #[description = "How many? Defaults to 1."]
    amount: Option<u8>
) -> Result<(), Error> {
    if let Some(pokemon) = ctx.data().pokemon.get(&pokemon.to_lowercase()) {
        ctx.say(build_encounter_string(pokemon, level, amount)).await?;
    } else {
        ctx.send(|b| {
            b.content(std::format!("Unable to find a pokemon named **{}**, sorry! If that wasn't a typo, maybe it isn't implemented yet?", pokemon));
            b.ephemeral(true)
        }).await?;
    }

    Ok(())
}

fn build_encounter_string(pokemon: &Pokemon, level: u8, amount: Option<u8>) -> impl Into<String> + Sized {
    let mut result = String::from("**-- WORK IN PROGRESS --**\n");
    for _ in 0..amount.unwrap_or(1) {
        let mon = EncounterMon::from_pokemon(pokemon, level);
        result.push_str(std::format!("{:?}", mon).as_str());
    }

    result
}

#[derive(Debug)]
struct EncounterMon {
    pub name: String,
    pub level: u8,
    pub rank: MysteryDungeonRank,
    pub ability: String,
    pub hp: u8,
    pub will: u8,
    pub strength: u8,
    pub dexterity: u8,
    pub vitality: u8,
    pub special: u8,
    pub insight: u8,
    pub moves: Vec<String>,
}

impl EncounterMon {
    pub fn from_pokemon(pokemon: &Pokemon, level: u8) -> Self {
        let mut result = EncounterMon {
            name: pokemon.name.clone(),
            level: level,
            rank: EncounterMon::get_rank_from_level(level),
            ability: EncounterMon::get_random_ability(pokemon),
            hp: 0,
            will: 0,
            strength: pokemon.strength.min,
            dexterity: pokemon.dexterity.min,
            vitality: pokemon.vitality.min,
            special: pokemon.special.min,
            insight: pokemon.insight.min,
            moves: Vec::new()
        };

        let mut remaining_stat_points = level + 3;
        while remaining_stat_points > 0 {
            result.increase_random_stat(pokemon);
            remaining_stat_points -= 1;
        }

        result.hp = (pokemon.base_hp + result.vitality) * 2;
        result.will = result.insight + 2;

        let available_moves = pokemon.moves.by_pokerole_rank
            .iter()
            .filter(|x| x.rank <= result.rank)
            .map(|x| x.name.clone());


        let move_count = result.insight + 2;
        result.moves = available_moves
            .choose_multiple(&mut thread_rng(), move_count as usize);

        result
    }

    fn get_random_ability(pokemon: &Pokemon) -> String {
        let rng = thread_rng().gen_range(0..100);
        if rng > 95 {
            if let Some(result) = &pokemon.hidden_ability {
                return result.clone();
            }
        }

        if rng > 43 {
            if let Some(result) = &pokemon.ability2 {
                return result.clone();
            }
        }

        pokemon.ability1.clone()
    }

    fn increase_random_stat(&mut self, pokemon: &Pokemon) {
        let rng = thread_rng().gen_range(0..=4);
        if rng == 0 && self.strength < pokemon.strength.max {
            self.strength += 1;
        } else if rng == 1 && self.dexterity < pokemon.dexterity.max {
            self.dexterity += 1;
        } else if rng == 2 && self.vitality < pokemon.vitality.max {
            self.vitality += 1;
        } else if rng == 3 && self.special < pokemon.special.max {
            self.special += 1;
        } else if rng == 4 && self.insight < pokemon.insight.max {
            self.insight += 1;
        } else {
            log::warn!("limit break not implemented for /encounter")
            // todo: limit break
        }
    }

    fn get_rank_from_level(level: u8) -> MysteryDungeonRank {
        match level {
            1 => MysteryDungeonRank::Bronze,
            2..=3 => MysteryDungeonRank::Silver,
            4..=7 => MysteryDungeonRank::Gold,
            8..=15 => MysteryDungeonRank::Platinum,
            _ => MysteryDungeonRank::Diamond,
        }
    }
}
