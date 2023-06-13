use rand::{Rng, thread_rng};
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use crate::commands::{Context, Error};
use crate::commands::autocompletion::autocomplete_pokemon;
use crate::data::pokemon::{Pokemon};
use crate::enums::{MysteryDungeonRank, Stat};


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

        let mut rng = thread_rng();
        let mut non_maxed_stat_points = vec!(Stat::Strength, Stat::Vitality, Stat::Dexterity, Stat::Special, Stat::Insight);
        let mut remaining_stat_points = level + 3;
        while remaining_stat_points > 0 {
            if let Some(mut stat) = non_maxed_stat_points.choose(&mut rng) {
                result.increase_stat(stat);

                if result.get_stat(stat) == pokemon.get_stat(stat).max {
                    let el_drop_o = *stat;
                    stat = &el_drop_o;
                    non_maxed_stat_points.retain(|x| x != stat);
                }
            }

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

    fn increase_stat(&mut self, stat: &Stat) {
        match stat {
            Stat::Strength => self.strength += 1,
            Stat::Dexterity => self.dexterity += 1,
            Stat::Vitality => self.vitality += 1,
            Stat::Special => self.special += 1,
            Stat::Insight => self.insight += 1,
            _ => panic!("Unexpected stat: {}", stat)
        };
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

    fn get_stat(&self, stat: &Stat) -> u8 {
        match stat {
            Stat::Strength => self.strength,
            Stat::Dexterity => self.dexterity,
            Stat::Vitality => self.vitality,
            Stat::Special => self.special,
            Stat::Insight => self.insight,
            _ => panic!("Unexpected stat: {}", stat)
        }
    }
}
