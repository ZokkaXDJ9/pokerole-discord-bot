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
    Flying, Poison,
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize)]
pub enum MysteryDungeonRank {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize)]
pub enum RegionalVariant {
    Alola,
    Galar,
    Hisui,
    Paldea,
}

#[derive(Debug, Clone, Copy, Deserialize, EnumString)]
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

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum PokemonGeneration {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9
}
