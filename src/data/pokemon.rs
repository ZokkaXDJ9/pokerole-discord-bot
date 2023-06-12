use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use std::sync::Arc;
use log::{error, warn};
use serde::Deserialize;
use crate::data::ability::Ability;
use crate::data::enums::poke_role_rank::PokeRoleRank;
use crate::data::parser::custom_data::custom_pokemon::{CustomPokemon, CustomPokemonMoves};
use crate::data::pokemon_api::pokemon_api_parser::PokemonApiData;
use crate::data::pokemon_api::PokemonApiId;
use crate::data::pokerole_data::raw_pokemon::{RawPokemonMoveLearnedByLevelUp, RawPokerolePokemon};
use crate::enums::{MysteryDungeonRank, PokemonType, RegionalVariant};

#[derive(Debug)]
pub struct Pokemon {
    pub number: u16,
    pub poke_api_id: PokemonApiId,
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

impl Pokemon {
    pub(crate) fn build_ability_string(&self, abilities: &Arc<HashMap<String, Ability>>) -> impl Into<String> + Sized {
        let mut result = std::format!("## {} Abilities\n", self.name);
        Pokemon::push_ability(&mut result, &self.ability1, abilities, "");
        if let Some(ability) = &self.ability2 {
            Pokemon::push_ability(&mut result, ability, abilities, "");
        }

        if let Some(ability) = &self.hidden_ability {
            Pokemon::push_ability(&mut result, ability, abilities, "(Hidden)");
        }

        if let Some(ability) = &self.event_abilities {
            Pokemon::push_ability(&mut result, ability, abilities, "(Event / Hidden)");
        }

        result
    }

    fn push_ability(result: &mut String, ability_name: &String, abilities: &Arc<HashMap<String, Ability>>, suffix: &str) {
        match abilities.get(ability_name.to_lowercase().as_str()) {
            None => result.push_str(std::format!("### {} {}\nNot implemented. :(\n", ability_name, suffix).as_str()),
            Some(ability) => result.push_str(std::format!("{}\n", ability.build_string(suffix).into()).as_str())
        };
    }
}

impl Pokemon {
    pub(crate) fn build_tm_move_string(&self) -> impl Into<String> + Sized {
        let mut result = std::format!("### {} [#{}]\n", self.name, self.number);
        if let Some(issue) = self.api_issue {
            if issue == ApiIssueType::FoundNothing {
                result.push_str("\nUnable to match any species to this particular pokemon when searching for TM Moves.");
            } else if issue == ApiIssueType::IsLegendary {
                result.push_str("\nToo lazy to be bothered to get this to work for legendary pokemon, sorry!");
            } else {
                result.push_str("\n**Struggling to match an exact species to this particular pokemon when searching for TM Moves. Take the values here with a grain of salt!**\n");
                self.append_all_learnable_moves(&mut result);
            }
        } else {
            self.append_all_learnable_moves(&mut result);
        }

        result
    }

    fn append_all_learnable_moves(&self, result: &mut String) {
        Pokemon::append_moves(result, "", "\n**TM Moves**\n", self.moves.by_machine.clone());
        Pokemon::append_moves(result, "", "\n**Egg Moves**\n", self.moves.by_egg.clone());
        Pokemon::append_moves(result, "", "\n**Tutor**\n", self.moves.by_tutor.clone());
        Pokemon::append_moves(result, "", "\n**Learned in Game through level up, but not here**\n", self.moves.by_level_up.iter()
            .filter(|x| self.moves.by_pokerole_rank.iter().all(|learn| learn.name.to_lowercase() != x.to_lowercase()))
            .cloned()
            .collect());
    }
}

// FIXME: Move these somewhere else. Also check out if we can use serenity::utils::parse_emoji for this.
const BRONZE: &str = "<:bronze_rank:1117520898426155019>";
const SILVER: &str = "<:silver_rank:1117520912531595375>";
const GOLD: &str = "<:gold_rank:1117520927245217872>";
const PLATINUM: &str = "<:platinum_rank:1117520966151585812>";
const DIAMOND: &str = "<:diamond_rank:1117520988956020806>";

impl Pokemon {
    pub(crate) fn build_move_string(&self) -> impl Into<String> + Sized {
        let mut result = std::format!("### {} [#{}]\n", self.name, self.number);
        self.filter_moves(&mut result, BRONZE, " **Bronze**\n", |x:&PokemonMoveLearnedByRank| x.rank == MysteryDungeonRank::Bronze);
        self.filter_moves(&mut result, SILVER, " **Silver**\n", |x:&PokemonMoveLearnedByRank| x.rank == MysteryDungeonRank::Silver);
        self.filter_moves(&mut result, GOLD, " **Gold**\n", |x:&PokemonMoveLearnedByRank| x.rank == MysteryDungeonRank::Gold);
        self.filter_moves(&mut result, PLATINUM, " **Platinum**\n", |x:&PokemonMoveLearnedByRank| x.rank == MysteryDungeonRank::Platinum);
        self.filter_moves(&mut result, DIAMOND, " **Diamond**\n", |x:&PokemonMoveLearnedByRank| x.rank == MysteryDungeonRank::Diamond);

        result
    }
    fn filter_moves<F>(&self, result: &mut String, emoji: &str, title: &str, filter: F)
        where F: Fn(&PokemonMoveLearnedByRank) -> bool {
        let moves = self.moves.by_pokerole_rank.iter()
            .filter(|x| filter(x))
            .map(|x| x.name.clone())
            .collect::<Vec<String>>();

        Pokemon::append_moves(result, emoji, title, moves);
    }

    fn append_moves(result: &mut String, emoji: &str, title: &str, moves: Vec<String>) {
        let text = moves.join("  |  ");

        if text.is_empty() {
            return;
        }

        result.push_str(emoji);
        result.push_str(title);
        result.push_str(&text);
        result.push('\n');
    }
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
            .replace('\'', "’") // Fixes Farfetch'd and Sirfetch'd
            .replace("Flabebe", "Flabébé")
            .replace("Nidoran M", "Nidoran♂")
            .replace("Nidoran F", "Nidoran♀")
            .replace("Mime Jr", "Mime Jr.")
            .replace("Ho-oh", "Ho-Oh");
        if let Some(value) = api.get(&fixed_name) {
            return (None, Some(value))
        }
        let options: Vec<String> = api.keys()
            .filter(|x| x.contains(fixed_name.split(' '.to_owned()).collect::<Vec<&str>>()[0]))
            .cloned()
            .collect();

        if options.is_empty() {
            error!("Found no matches for {}", fixed_name);
            return (Some(ApiIssueType::FoundNothing), None);
        }

        if options.len() == 1 {
            return (None, api.get(options.first().unwrap()));
        }

        if fixed_name.contains("Form)") {
            // What we want is between "<name> (" and " Form)". Bet we can search the keys for that and find a unique match.
            let form = fixed_name.split('(').collect::<Vec<&str>>()[1].replace(" Form)", "");
            let form_options: Vec<String> = options.iter().filter(|x| x.contains(&form) && !x.contains("Gigantamax")).map(|x| x.to_owned()).collect();

            if form_options.len() == 1 {
                return (None, api.get(form_options.first().unwrap()));
            }
        }

        warn!("Found multiple matches for {}", name);

        (Some(ApiIssueType::Form), api.get(options.first().unwrap()))
    }


    fn get_api_entry<'a>(name: &str, api: &'a HashMap<String, PokemonApiData>, regional_variant: &Option<RegionalVariant>)
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
            moves = LearnablePokemonMoves::create_from(
                raw.moves.iter().map(PokemonMoveLearnedByRank::new).collect(),
                api_data.learnable_moves.level_up.iter().map(|x| x.move_name.to_owned()).collect(),
                api_data.learnable_moves.machine.iter().map(|x| x.move_name.to_owned()).collect(),
                api_data.learnable_moves.tutor.iter().map(|x| x.move_name.to_owned()).collect(),
                api_data.learnable_moves.egg.iter().map(|x| x.move_name.to_owned()).collect()
            );
        } else {
            moves = LearnablePokemonMoves::create_from(
                raw.moves.iter().map(PokemonMoveLearnedByRank::new).collect(),
                vec![],
                vec![],
                vec![],
                vec![]
            );
        }

        let api_id = match api_option {
            None => PokemonApiId(raw.number),
            Some(item) => PokemonApiId(item.pokemon_id.0),
        };

        Pokemon {
            number: raw.number,
            poke_api_id: api_id,
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
        let regional_variant = raw.variant;

        let (api_issue, api_option) = Pokemon::get_api_entry(&raw.name, api, &regional_variant);
        let api_data = api_option.unwrap_or_else(|| panic!("API Data should ALWAYS be found for custom mons. {}", raw.name));

        let moves = LearnablePokemonMoves::create_from(
            Pokemon::moves_from_custom(&raw.moves),
            api_data.learnable_moves.level_up.iter().map(|x| x.move_name.to_owned()).collect(),
            api_data.learnable_moves.machine.iter().map(|x| x.move_name.to_owned()).collect(),
            api_data.learnable_moves.tutor.iter().map(|x| x.move_name.to_owned()).collect(),
            api_data.learnable_moves.egg.iter().map(|x| x.move_name.to_owned()).collect()
        );

        Pokemon {
            number: raw.number,
            poke_api_id: PokemonApiId(api_data.pokemon_id.0),
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

        Some(PokemonType::from_str(&raw).unwrap())
    }

    fn parse_ability(raw: String) -> Option<String> {
        if raw.is_empty() {
            return None;
        }

        Some(raw)
    }

    pub fn build_stats_string(&self) -> String {
        let mut result = std::format!("### {} [#{}]\n", self.name, self.number);
        result.push_str(&std::format!("{}   |   {}\n",
                                      self.height,
                                      self.weight));
        result.push_str("**Type**: ");
        result.push_str(std::format!("{:?}", self.type1).as_str());
        if let Some(type2) = self.type2 {
            result.push_str(std::format!(" / {:?}", type2).as_str())
        }
        result.push('\n');

        result.push_str(&std::format!("**Base HP**: {}\n", self.base_hp));

        self.strength.append_stat_string(&mut result, "Strength");
        self.dexterity.append_stat_string(&mut result, "Dexterity");
        self.vitality.append_stat_string(&mut result, "Vitality");
        self.special.append_stat_string(&mut result, "Special");
        self.insight.append_stat_string(&mut result, "Insight");

        result.push_str("**Ability**: ");
        result.push_str(&self.ability1);
        if let Some(ability2) = &self.ability2 {
            result.push_str(&std::format!(" / {}", ability2))
        }

        if let Some(hidden) = &self.hidden_ability {
            result.push_str(&std::format!(" ({})", hidden))
        }

        if let Some(event) = &self.event_abilities {
            result.push_str(&std::format!(" ({})", event))
        }

        result
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
        let splits: Vec<&str> = raw.split('/').collect();
        let min = u8::from_str(splits[0]).expect("Data is always right, riight?");
        let max = u8::from_str(splits[1]).expect("Data is always right, riiiight?");

        Stat::new(min, max)
    }

    pub fn append_stat_string(&self, result: &mut String, stat_name: &str) {
        result.push_str(&std::format!("**{}**: ", stat_name));

        for _ in 0..self.min {
            result.push('⬤');
        }
        for _ in 0..self.max-self.min {
            result.push('⭘');
        }

        result.push_str(&std::format!(" `{}/{}`\n", self.min, self.max));
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Height {
    pub meters: f32,
    pub feet: f32,
}

impl fmt::Display for Height{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}m / {:.2}ft", self.meters, self.feet)
    }
}

impl Height {
    pub fn scale(&self, percentage: u8) -> Height{
        Height {
            meters: self.meters * (percentage as f32 * 0.01),
            feet: self.feet * (percentage as f32 * 0.01),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Weight {
    pub kilograms: f32,
    pub pounds: f32,
}

impl fmt::Display for Weight{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}kg / {:.1}lbs", self.kilograms, self.pounds)
    }
}

impl Weight {
    pub fn scale(&self, percentage: u8) -> Weight{
        Weight {
            kilograms: self.kilograms * (percentage as f32 * 0.01),
            pounds: self.pounds * (percentage as f32 * 0.01),
        }
    }
}

#[derive(Debug)]
pub struct LearnablePokemonMoves {
    pub by_pokerole_rank: Vec<PokemonMoveLearnedByRank>,
    pub by_level_up: Vec<String>,
    pub by_machine: Vec<String>,
    pub by_tutor: Vec<String>,
    pub by_egg: Vec<String>,
}

impl LearnablePokemonMoves {
    pub fn create_from(by_pokerole_rank: Vec<PokemonMoveLearnedByRank>,
                       by_level_up: Vec<String>,
                       by_machine: Vec<String>,
                       by_tutor: Vec<String>,
                       by_egg: Vec<String>) -> Self {

        let mut result = LearnablePokemonMoves {
            by_pokerole_rank,
            by_level_up,
            by_machine,
            by_tutor,
            by_egg,
        };

        result.by_pokerole_rank.sort_by(|a, b| a.name.cmp(&b.name));
        result.by_level_up.sort();
        result.by_machine.sort();
        result.by_tutor.sort();
        result.by_egg.sort();

        result
    }
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
