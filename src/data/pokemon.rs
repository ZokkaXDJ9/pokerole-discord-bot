use std::collections::HashMap;
use std::str::FromStr;
use log::{error, warn};
use serde::Deserialize;
use crate::data::enums::poke_role_rank::PokeRoleRank;
use crate::data::parser::custom_data::custom_pokemon::{CustomPokemon, CustomPokemonMoves};
use crate::data::pokemon_api::pokemon_api_parser::PokemonApiData;
use crate::data::pokerole_data::raw_pokemon::{RawPokemonMoveLearnedByLevelUp, RawPokerolePokemon};
use crate::enums::{MysteryDungeonRank, PokemonType, RegionalVariant};

#[derive(Debug)]
pub struct Pokemon {
    pub number: u16,
    pub regional_variant: Option<RegionalVariant>,
    pub api_issue: Option<ApiIssueType>,
    pub name: String,
    pub type1: PokemonType,
    pub type2: Option<PokemonType>,
    pub base_hp: u8,
    pub strength: Stat,
    pub dexterity: Stat,
    pub vitality: Stat,
    pub special: Stat,
    pub insight: Stat,
    pub ability1: String,
    pub ability2: Option<String>,
    pub hidden_ability: Option<String>,
    pub event_abilities: Option<String>,
    pub height: Height,
    pub weight: Weight,
    pub dex_description: String,
    pub moves: LearnablePokemonMoves,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize)]
pub enum ApiIssueType {
    FoundNothing,
    Form,
    IsLegendary,
}

impl Pokemon {
    fn try_find<'a>(name: &str, api: &'a HashMap<String, PokemonApiData>)
        -> (Option<ApiIssueType>, Option<&'a PokemonApiData>) {
        if let Some(value) = api.get(name) {
            return (None, Some(value))
        }
        let fixed_name = name
            .replace("'", "’") // Fixes Farfetch'd and Sirfetch'd
            .replace("Flabebe", "Flabébé")
            .replace("Nidoran M", "Nidoran♂")
            .replace("Nidoran F", "Nidoran♀")
            .replace("Mime Jr", "Mime Jr.")
            .replace("Ho-oh", "Ho-Oh");
        if let Some(value) = api.get(&fixed_name) {
            return (None, Some(value))
        }
        let options: Vec<String> = api.keys()
            .filter(|x| x.contains(&fixed_name.split(' '.to_owned()).collect::<Vec<&str>>()[0]))
            .map(|x| x.clone())
            .collect();

        if options.len() == 0 {
            error!("Found no matches for {}", fixed_name);
            return (Some(ApiIssueType::FoundNothing), None);
        }

        if options.len() == 1 {
            return (None, api.get(options.first().unwrap()));
        }

        if fixed_name.contains("Form)") {
            // What we want is between "<name> (" and " Form)". Bet we can search the keys for that and find a unique match.
            let form = fixed_name.split("(").collect::<Vec<&str>>()[1].replace(" Form)", "");
            let form_options: Vec<String> = options.iter().filter(|x| x.contains(&form) && !x.contains("Gigantamax")).map(|x| x.to_owned()).collect();

            if form_options.len() == 1 {
                return (None, api.get(form_options.first().unwrap()));
            }
        }

        warn!("Found multiple matches for {}", name);

        (Some(ApiIssueType::Form), api.get(options.first().unwrap()))
    }


    fn get_api_entry<'a>(name: &String, api: &'a HashMap<String, PokemonApiData>, regional_variant: &Option<RegionalVariant>)
        -> (Option<ApiIssueType>, Option<&'a PokemonApiData>) {
        match regional_variant {
            None => Pokemon::try_find(name, api),
            Some(variant) => {
                // We can either replace <pokemon name>(Galarian Form) with Galarian <Pokemon name>
                // Or search for the respective form by using the <pokemon name> and form_id.
                // pokemon.csv maps pokemon-id to pokedex #, that way we could figure out how many forms a specific mon has and what they are called
                match variant {
                    RegionalVariant::Alola => Pokemon::try_find(&(String::from("Alolan ") + name.split(" (Alolan Form)").collect::<Vec<&str>>()[0]), api),
                    RegionalVariant::Galar => Pokemon::try_find(&(String::from("Galarian ") + name.split(" (Galarian Form)").collect::<Vec<&str>>()[0]), api),
                    RegionalVariant::Hisui => Pokemon::try_find(&(String::from("Hisuian ") + name.split(" (Hisuian Form)").collect::<Vec<&str>>()[0]), api),
                    RegionalVariant::Paldea => Pokemon::try_find(&(String::from("Paldean ") + name.split(" (Paldean Form)").collect::<Vec<&str>>()[0]), api)
                }
            }
        }
    }

    pub(in crate::data) fn new(raw: &RawPokerolePokemon, api: &HashMap<String, PokemonApiData>) -> Self {
        let regional_variant= Pokemon::parse_variant(&raw.dex_id);

        let (api_issue, api_option) = match raw.legendary {
            false => Pokemon::get_api_entry(&raw.name, api, &regional_variant),
            true => (Some(ApiIssueType::IsLegendary), None)
        };

        let moves;
        if let Some(api_data) = api_option {
            moves = LearnablePokemonMoves {
                by_pokerole_rank: raw.moves.iter().map(|x| PokemonMoveLearnedByRank::new(x)).collect(),
                by_level_up: api_data.learnable_moves.level_up.iter().map(|x| x.move_name.to_owned()).collect(),
                by_machine: api_data.learnable_moves.machine.iter().map(|x| x.move_name.to_owned()).collect(),
                by_tutor: api_data.learnable_moves.tutor.iter().map(|x| x.move_name.to_owned()).collect(),
                by_egg: api_data.learnable_moves.egg.iter().map(|x| x.move_name.to_owned()).collect()
            };
        } else {
            moves = LearnablePokemonMoves {
                by_pokerole_rank: raw.moves.iter().map(|x| PokemonMoveLearnedByRank::new(x)).collect(),
                by_level_up: vec![],
                by_machine: vec![],
                by_tutor: vec![],
                by_egg: vec![]
            };
        }

        Pokemon {
            number: raw.number,
            name: raw.name.clone(),
            regional_variant,
            api_issue,
            type1: Pokemon::parse_type(raw.type1.clone()).unwrap(),
            type2: Pokemon::parse_type(raw.type2.clone()),
            base_hp: raw.base_hp,
            strength: Stat::new(raw.strength, raw.max_strength),
            dexterity: Stat::new(raw.dexterity, raw.max_dexterity),
            vitality: Stat::new(raw.vitality, raw.max_vitality),
            special: Stat::new(raw.special, raw.max_special),
            insight: Stat::new(raw.insight, raw.max_insight),
            ability1: raw.ability1.clone(),
            ability2: Pokemon::parse_ability(raw.ability2.clone()),
            hidden_ability: Pokemon::parse_ability(raw.hidden_ability.clone()),
            event_abilities: Pokemon::parse_ability(raw.event_abilities.clone()),
            height: raw.height.clone(),
            weight: raw.weight.clone(),
            dex_description: raw.dex_description.clone(),
            moves
        }
    }

    fn moves_from_custom(moves: &CustomPokemonMoves) -> Vec<PokemonMoveLearnedByRank> {
        let mut result = Vec::new();

        for x in &moves.bronze {
            result.push(PokemonMoveLearnedByRank {rank: MysteryDungeonRank::Bronze, name: x.clone()})
        }
        for x in &moves.silver {
            result.push(PokemonMoveLearnedByRank {rank: MysteryDungeonRank::Silver, name: x.clone()})
        }
        for x in &moves.gold {
            result.push(PokemonMoveLearnedByRank {rank: MysteryDungeonRank::Gold, name: x.clone()})
        }
        for x in &moves.platinum {
            result.push(PokemonMoveLearnedByRank {rank: MysteryDungeonRank::Platinum, name: x.clone()})
        }
        for x in &moves.diamond {
            result.push(PokemonMoveLearnedByRank {rank: MysteryDungeonRank::Diamond, name: x.clone()})
        }

        result
    }

    pub(in crate::data) fn from_custom_data(raw: &CustomPokemon, api: &HashMap<String, PokemonApiData>) -> Self {
        let regional_variant= None;

        let (api_issue, api_option) = Pokemon::get_api_entry(&raw.name, api, &regional_variant);
        let api_data = api_option.expect(&std::format!("API Data should ALWAYS be found for custom mons. {}", raw.name));

        let moves = LearnablePokemonMoves {
            by_pokerole_rank: Pokemon::moves_from_custom(&raw.moves),
            by_level_up: api_data.learnable_moves.level_up.iter().map(|x| x.move_name.to_owned()).collect(),
            by_machine: api_data.learnable_moves.machine.iter().map(|x| x.move_name.to_owned()).collect(),
            by_tutor: api_data.learnable_moves.tutor.iter().map(|x| x.move_name.to_owned()).collect(),
            by_egg: api_data.learnable_moves.egg.iter().map(|x| x.move_name.to_owned()).collect()
        };

        Pokemon {
            number: raw.number,
            name: raw.name.clone(),
            regional_variant,
            api_issue,
            type1: api_data.type1,
            type2: api_data.type2,
            base_hp: raw.base_hp,
            strength: Stat::from_str(&raw.strength),
            dexterity: Stat::from_str(&raw.dexterity),
            vitality: Stat::from_str(&raw.vitality),
            special: Stat::from_str(&raw.special),
            insight: Stat::from_str(&raw.insight),
            ability1: api_data.ability1.clone(),
            ability2: api_data.ability2.clone(),
            hidden_ability: api_data.ability_hidden.clone(),
            event_abilities: api_data.ability_event.clone(),
            height: api_data.height.clone(),
            weight: api_data.weight.clone(),
            dex_description: String::from("Dex coming soon"),// raw.dex_description.clone(),
            moves
        }
    }

    fn parse_variant(dex_id: &str) -> Option<RegionalVariant> {
        if dex_id.contains('A') {
            return Some(RegionalVariant::Alola);
        }
        if dex_id.contains('G') {
            return Some(RegionalVariant::Galar);
        }
        if dex_id.contains('H') {
            return Some(RegionalVariant::Hisui);
        }
        if dex_id.contains('P') {
            return Some(RegionalVariant::Paldea);
        }

        None
    }

    fn parse_type(raw: String) -> Option<PokemonType> {
        if raw.is_empty() {
            return None;
        }

        return Some(PokemonType::from_str(&raw).unwrap());
    }

    fn parse_ability(raw: String) -> Option<String> {
        if raw.is_empty() {
            return None;
        }

        return Some(raw);
    }
}

#[derive(Debug)]
pub struct Stat {
    pub min: u8,
    pub max: u8,
}

impl Stat {
    fn new(min: u8, max: u8) -> Self {
        Stat {min, max}
    }

    fn from_str(raw: &str) -> Self {
        let splits: Vec<&str> = raw.split("/").collect();
        let min = u8::from_str(splits[0]).expect("Data is always right, riight?");
        let max = u8::from_str(splits[1]).expect("Data is always right, riiiight?");

        Stat::new(min, max)
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Height {
    pub meters: f32,
    pub feet: f32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Weight {
    pub kilograms: f32,
    pub pounds: f32,
}

#[derive(Debug)]
pub struct LearnablePokemonMoves {
    pub by_pokerole_rank: Vec<PokemonMoveLearnedByRank>,
    pub by_level_up: Vec<String>,
    pub by_machine: Vec<String>,
    pub by_tutor: Vec<String>,
    pub by_egg: Vec<String>,
}

#[derive(Debug)]
pub struct PokemonMoveLearnedByRank {
    pub rank: MysteryDungeonRank,
    pub name: String
}

impl PokemonMoveLearnedByRank {
    pub(in crate::data) fn new(raw: &RawPokemonMoveLearnedByLevelUp) -> Self {
        let rank = match raw.learned {
            PokeRoleRank::Starter => MysteryDungeonRank::Bronze,
            PokeRoleRank::Beginner => MysteryDungeonRank::Bronze,
            PokeRoleRank::Amateur => MysteryDungeonRank::Silver,
            PokeRoleRank::Ace => MysteryDungeonRank::Gold,
            PokeRoleRank::Pro => MysteryDungeonRank::Platinum,
            PokeRoleRank::Master => MysteryDungeonRank::Diamond,
            PokeRoleRank::Champion => MysteryDungeonRank::Diamond,
        };

        PokemonMoveLearnedByRank {rank, name: raw.name.clone()}
    }
}
