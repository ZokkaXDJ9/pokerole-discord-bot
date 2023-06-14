use std::fmt;
use std::fmt::{Formatter};
use serde::Deserialize;
use strum_macros::{EnumIter, EnumString};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, EnumString, Hash, EnumIter)]
pub enum PokemonType {
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
    Shadow,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub enum MoveType {
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
    Any,
    Typeless,
}

impl MoveType {
    pub fn has_stab(&self, poke_type: &Option<PokemonType>) -> bool {
        if let Some(poke_type) = poke_type {
            match poke_type {
                PokemonType::Normal => self == &MoveType::Normal,
                PokemonType::Fighting => self == &MoveType::Fighting,
                PokemonType::Flying => self == &MoveType::Flying,
                PokemonType::Poison => self == &MoveType::Poison,
                PokemonType::Ground => self == &MoveType::Ground,
                PokemonType::Rock => self == &MoveType::Rock,
                PokemonType::Bug => self == &MoveType::Bug,
                PokemonType::Ghost => self == &MoveType::Ghost,
                PokemonType::Steel => self == &MoveType::Steel,
                PokemonType::Fire => self == &MoveType::Fire,
                PokemonType::Water => self == &MoveType::Water,
                PokemonType::Grass => self == &MoveType::Grass,
                PokemonType::Electric => self == &MoveType::Electric,
                PokemonType::Psychic => self == &MoveType::Psychic,
                PokemonType::Ice => self == &MoveType::Ice,
                PokemonType::Dragon => self == &MoveType::Dragon,
                PokemonType::Dark => self == &MoveType::Dark,
                PokemonType::Fairy => self == &MoveType::Fairy,
                PokemonType::Shadow => false
            }
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum MoveCategory {
    Physical,
    Special,
    #[serde(rename = "Physical/special")] /// Only used for struggle and tera blast
    PhysicalOrSpecial,
    Support,
}

impl fmt::Display for MoveCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            MoveCategory::Physical => "Physical",
            MoveCategory::Special => "Special",
            MoveCategory::PhysicalOrSpecial => "Physical / Special",
            MoveCategory::Support => "Support"
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Deserialize)]
pub enum MysteryDungeonRank {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond
}

impl MysteryDungeonRank {
    pub fn die_count(&self) -> u8 {
        match self {
            MysteryDungeonRank::Bronze => 1,
            MysteryDungeonRank::Silver => 2,
            MysteryDungeonRank::Gold => 3,
            MysteryDungeonRank::Platinum => 4,
            MysteryDungeonRank::Diamond => 5,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize)]
pub enum RegionalVariant {
    Alola,
    Galar,
    Hisui,
    Paldea,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, EnumString)]
pub enum Stat {
    Strength,
    Dexterity,
    Vitality,
    Special,
    Insight,
    /// Struggle
    StrengthOrSpecial,
    /// Copycat
    Copy
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SocialStat {
    Tough,
    Cool,
    Beauty,
    Clever,
    Cute,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize)]
pub enum Gender {
    Genderless,
    Male,
    Female,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Gender::Genderless => "–",
            Gender::Male => "M",
            Gender::Female => "F",
        })
    }
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Stat::Strength => "Strength",
            Stat::Dexterity => "Dexterity",
            Stat::Vitality => "Vitality",
            Stat::Special => "Special",
            Stat::Insight => "Insight",
            Stat::StrengthOrSpecial => "Strength / Special",
            Stat::Copy => "Copy",
        })
    }
}

#[derive(Debug, Clone, Copy, EnumString)]
pub enum CombatOrSocialStat {
    Strength,
    Dexterity,
    Vitality,
    Special,
    Insight,
    Tough,
    Cool,
    Beauty,
    Clever,
    Cute,
    Brawl,
    Channel,
    Clash,
    Evasion,
    Alert,
    Athletic,
    Nature,
    Stealth,
    Allure,
    Etiquette,
    Intimidate,
    Perform,
    Will,
    Copied,
    ToughOrCute,
    MissingBeauty,
    BrawlOrChannel,
    Varies,
    Medicine,
    Empathy,
    Rank,
}

impl fmt::Display for CombatOrSocialStat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            CombatOrSocialStat::Strength => "Strength",
            CombatOrSocialStat::Dexterity => "Dexterity",
            CombatOrSocialStat::Vitality => "Vitality",
            CombatOrSocialStat::Special => "Special",
            CombatOrSocialStat::Insight => "Insight",
            CombatOrSocialStat::Tough => "Tough",
            CombatOrSocialStat::Cool => "Cool",
            CombatOrSocialStat::Beauty => "Beauty",
            CombatOrSocialStat::Clever => "Clever",
            CombatOrSocialStat::Cute => "Cute",
            CombatOrSocialStat::Brawl => "Brawl",
            CombatOrSocialStat::Channel => "Channel",
            CombatOrSocialStat::Clash => "Clash",
            CombatOrSocialStat::Evasion => "Evasion",
            CombatOrSocialStat::Alert => "Alert",
            CombatOrSocialStat::Athletic => "Athletic",
            CombatOrSocialStat::Nature => "Nature",
            CombatOrSocialStat::Stealth => "Stealth",
            CombatOrSocialStat::Allure => "Allure",
            CombatOrSocialStat::Etiquette => "Etiquette",
            CombatOrSocialStat::Intimidate => "Intimidate",
            CombatOrSocialStat::Perform => "Perform",
            CombatOrSocialStat::Will => "Will",
            CombatOrSocialStat::Copied => "Copied",
            CombatOrSocialStat::ToughOrCute => "Tough / Cute",
            CombatOrSocialStat::MissingBeauty => "5 - Beauty",
            CombatOrSocialStat::BrawlOrChannel => "Brawl / Channel",
            CombatOrSocialStat::Varies => "Varies",
            CombatOrSocialStat::Medicine => "Medicine",
            CombatOrSocialStat::Empathy => "Empathy",
            CombatOrSocialStat::Rank => "Rank",
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HappinessDamageModifier {
    Happiness,
    MissingHappiness
}

impl fmt::Display for HappinessDamageModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            HappinessDamageModifier::Happiness => "Happiness",
            HappinessDamageModifier::MissingHappiness => "Missing Happiness"
        })
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialOrd, PartialEq)]
pub enum PokemonGeneration {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}
