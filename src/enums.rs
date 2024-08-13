use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::Iterator;
use std::string::ToString;

use poise::ChoiceParameter;
use serde::Deserialize;
use strum_macros::{EnumIter, EnumString, FromRepr};

use crate::emoji;

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

impl Display for PokemonType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            PokemonType::Normal => "<:typenormal:1272535965893791824> Normal",
            PokemonType::Fighting => "<:typefighting:1272535949569429586> Fighting",
            PokemonType::Flying => "<:typeflying:1272536305380753440> Flying",
            PokemonType::Poison => "<:typepoison:1272536309147238440> Poison",
            PokemonType::Ground => "<:typeground:1272535961682579496> Ground",
            PokemonType::Rock => "<:typerock:1272535973481283596> Rock",
            PokemonType::Bug => "<:typebug:1272535941420027924> Bug",
            PokemonType::Ghost => "<:typeghost:1272535956733300879> Ghost",
            PokemonType::Steel => "<:typesteel:1272536310984212491> Steel",
            PokemonType::Fire => "<:typefire:1272535951129968780> Fire",
            PokemonType::Water => "<:typewater:1272535976794652673> Water",
            PokemonType::Grass => "<:typegrass:1272535959677960222> Grass",
            PokemonType::Electric => "<:typeelectric:1272535946788606123> Electric",
            PokemonType::Psychic => "<:typepsychic:1272535970897592330> Psychic",
            PokemonType::Ice => "<:typeice:1272536307276709898> Ice",
            PokemonType::Dragon => "<:typedragon:1272535944804962335> Dragon",
            PokemonType::Dark => "<:typedark:1272535943060000800> Dark",
            PokemonType::Fairy => "<:typefairy:1272535948357537894> Fairy",
            PokemonType::Shadow => "Shadow",
        })
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, EnumString, Hash, EnumIter)]
#[strum(ascii_case_insensitive)]
pub enum PokemonTypeWithoutShadow {
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
}

impl PokemonTypeWithoutShadow {
    pub fn get_names_vec() -> Vec<String> {
        vec![
            String::from("Normal"),
            String::from("Fighting"),
            String::from("Flying"),
            String::from("Poison"),
            String::from("Ground"),
            String::from("Rock"),
            String::from("Bug"),
            String::from("Ghost"),
            String::from("Steel"),
            String::from("Fire"),
            String::from("Water"),
            String::from("Grass"),
            String::from("Electric"),
            String::from("Psychic"),
            String::from("Ice"),
            String::from("Dragon"),
            String::from("Dark"),
            String::from("Fairy"),
        ]
    }

    pub fn get_tera_unlocked_column(&self) -> &str {
        match self {
            PokemonTypeWithoutShadow::Normal => "tera_unlocked_normal",
            PokemonTypeWithoutShadow::Fighting => "tera_unlocked_fighting",
            PokemonTypeWithoutShadow::Flying => "tera_unlocked_flying",
            PokemonTypeWithoutShadow::Poison => "tera_unlocked_poison",
            PokemonTypeWithoutShadow::Ground => "tera_unlocked_ground",
            PokemonTypeWithoutShadow::Rock => "tera_unlocked_rock",
            PokemonTypeWithoutShadow::Bug => "tera_unlocked_bug",
            PokemonTypeWithoutShadow::Ghost => "tera_unlocked_ghost",
            PokemonTypeWithoutShadow::Steel => "tera_unlocked_steel",
            PokemonTypeWithoutShadow::Fire => "tera_unlocked_fire",
            PokemonTypeWithoutShadow::Water => "tera_unlocked_water",
            PokemonTypeWithoutShadow::Grass => "tera_unlocked_grass",
            PokemonTypeWithoutShadow::Electric => "tera_unlocked_electric",
            PokemonTypeWithoutShadow::Psychic => "tera_unlocked_psychic",
            PokemonTypeWithoutShadow::Ice => "tera_unlocked_ice",
            PokemonTypeWithoutShadow::Dragon => "tera_unlocked_dragon",
            PokemonTypeWithoutShadow::Dark => "tera_unlocked_dark",
            PokemonTypeWithoutShadow::Fairy => "tera_unlocked_fairy",
        }
    }

    pub fn get_tera_used_column(&self) -> &str {
        match self {
            PokemonTypeWithoutShadow::Normal => "tera_used_normal",
            PokemonTypeWithoutShadow::Fighting => "tera_used_fighting",
            PokemonTypeWithoutShadow::Flying => "tera_used_flying",
            PokemonTypeWithoutShadow::Poison => "tera_used_poison",
            PokemonTypeWithoutShadow::Ground => "tera_used_ground",
            PokemonTypeWithoutShadow::Rock => "tera_used_rock",
            PokemonTypeWithoutShadow::Bug => "tera_used_bug",
            PokemonTypeWithoutShadow::Ghost => "tera_used_ghost",
            PokemonTypeWithoutShadow::Steel => "tera_used_steel",
            PokemonTypeWithoutShadow::Fire => "tera_used_fire",
            PokemonTypeWithoutShadow::Water => "tera_used_water",
            PokemonTypeWithoutShadow::Grass => "tera_used_grass",
            PokemonTypeWithoutShadow::Electric => "tera_used_electric",
            PokemonTypeWithoutShadow::Psychic => "tera_used_psychic",
            PokemonTypeWithoutShadow::Ice => "tera_used_ice",
            PokemonTypeWithoutShadow::Dragon => "tera_used_dragon",
            PokemonTypeWithoutShadow::Dark => "tera_used_dark",
            PokemonTypeWithoutShadow::Fairy => "tera_used_fairy",
        }
    }
}

impl Display for PokemonTypeWithoutShadow {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            PokemonTypeWithoutShadow::Normal => "<:typenormal:1272535965893791824> Normal",
            PokemonTypeWithoutShadow::Fighting => "<:typefighting:1272535949569429586> Fighting",
            PokemonTypeWithoutShadow::Flying => "<:typeflying:1272536305380753440> Flying",
            PokemonTypeWithoutShadow::Poison => "<:typepoison:1272536309147238440> Poison",
            PokemonTypeWithoutShadow::Ground => "<:typeground:1272535961682579496> Ground",
            PokemonTypeWithoutShadow::Rock => "<:typerock:1272535973481283596> Rock",
            PokemonTypeWithoutShadow::Bug => "<:typebug:1272535941420027924> Bug",
            PokemonTypeWithoutShadow::Ghost => "<:typeghost:1272535956733300879> Ghost",
            PokemonTypeWithoutShadow::Steel => "<:typesteel:1272536310984212491> Steel",
            PokemonTypeWithoutShadow::Fire => "<:typefire:1272535951129968780> Fire",
            PokemonTypeWithoutShadow::Water => "<:typewater:1272535976794652673> Water",
            PokemonTypeWithoutShadow::Grass => "<:typegrass:1272535959677960222> Grass",
            PokemonTypeWithoutShadow::Electric => "<:typeelectric:1272535946788606123> Electric",
            PokemonTypeWithoutShadow::Psychic => "<:typepsychic:1272535970897592330> Psychic",
            PokemonTypeWithoutShadow::Ice => "<:typeice:1272536307276709898> Ice",
            PokemonTypeWithoutShadow::Dragon => "<:typedragon:1272535944804962335> Dragon",
            PokemonTypeWithoutShadow::Dark => "<:typedark:1272535943060000800> Dark",
            PokemonTypeWithoutShadow::Fairy => "<:typefairy:1272535948357537894> Fairy",
        })
    }
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

impl fmt::Display for MoveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            MoveType::Normal => "<:typenormal:1272535965893791824> Normal",
            MoveType::Fighting => "<:typefighting:1272535949569429586> Fighting",
            MoveType::Flying => "<:typeflying:1272536305380753440> Flying",
            MoveType::Poison => "<:typepoison:1272536309147238440> Poison",
            MoveType::Ground => "<:typeground:1272535961682579496> Ground",
            MoveType::Rock => "<:typerock:1272535973481283596> Rock",
            MoveType::Bug => "<:typebug:1272535941420027924> Bug",
            MoveType::Ghost => "<:typeghost:1272535956733300879> Ghost",
            MoveType::Steel => "<:typesteel:1272536310984212491> Steel",
            MoveType::Fire => "<:typefire:1272535951129968780> Fire",
            MoveType::Water => "<:typewater:1272535976794652673> Water",
            MoveType::Grass => "<:typegrass:1272535959677960222> Grass",
            MoveType::Electric => "<:typeelectric:1272535946788606123> Electric",
            MoveType::Psychic => "<:typepsychic:1272535970897592330> Psychic",
            MoveType::Ice => "<:typeice:1272536307276709898> Ice",
            MoveType::Dragon => "<:typedragon:1272535944804962335> Dragon",
            MoveType::Dark => "<:typedark:1272535943060000800> Dark",
            MoveType::Fairy => "<:typefairy:1272535948357537894> Fairy",
            MoveType::Any => "Any",
            MoveType::Typeless => "Typeless",
        })
    }
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
                PokemonType::Shadow => false,
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
    #[serde(rename = "Physical/special")]
    /// Only used for struggle and tera blast
    PhysicalOrSpecial,
    Support,
}

impl fmt::Display for MoveCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            MoveCategory::Physical => "<:movephysical:1272535935279435968> Physical",
            MoveCategory::Special => "<:movespecial:1272535937104220180> Special",
            MoveCategory::PhysicalOrSpecial => "<:movephysical:1272535935279435968> Physical / <:movespecial:1272535937104220180> Special",
            MoveCategory::Support => "<:movestatus:1272535939465478235> Support",
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Deserialize)]
pub enum MysteryDungeonRank {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
}

impl Display for MysteryDungeonRank {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MysteryDungeonRank::Bronze => write!(f, "<:badgebronze:1272532685197152349> Bronze Rank"),
            MysteryDungeonRank::Silver => write!(f, "<:badgesilver:1272533590697185391> Silver Rank"),
            MysteryDungeonRank::Gold => write!(f, "<:badgegold:1272532681992962068> Gold Rank"),
            MysteryDungeonRank::Platinum => write!(f, "<:badgeplatinum:1272533593750507570> Platinum Rank"),
            MysteryDungeonRank::Diamond => write!(f, "<:badgediamond:1272532683431481445> Diamond Rank"),
        }
    }
}

impl MysteryDungeonRank {
    pub fn name_without_emoji(&self) -> &str {
        match self {
            MysteryDungeonRank::Bronze => "Bronze Rank",
            MysteryDungeonRank::Silver => "Silver Rank",
            MysteryDungeonRank::Gold => "Gold Rank",
            MysteryDungeonRank::Platinum => "Platinum Rank",
            MysteryDungeonRank::Diamond => "Diamond Rank",
        }
    }

    pub fn die_count(&self) -> u8 {
        match self {
            MysteryDungeonRank::Bronze => 1,
            MysteryDungeonRank::Silver => 2,
            MysteryDungeonRank::Gold => 3,
            MysteryDungeonRank::Platinum => 4,
            MysteryDungeonRank::Diamond => 5,
        }
    }

    pub fn from_level(level: u8) -> Self {
        match level {
            1 => MysteryDungeonRank::Bronze,
            2..=3 => MysteryDungeonRank::Silver,
            4..=7 => MysteryDungeonRank::Gold,
            8..=15 => MysteryDungeonRank::Platinum,
            _ => MysteryDungeonRank::Diamond,
        }
    }

    pub fn emoji_string(&self) -> &str {
        match self {
            MysteryDungeonRank::Bronze => "<:badgebronze:1272532685197152349>",
            MysteryDungeonRank::Silver => "<:badgesilver:1272533590697185391>",
            MysteryDungeonRank::Gold => "<:badgegold:1272532681992962068>",
            MysteryDungeonRank::Platinum => "<:badgeplatinum:1272533593750507570>",
            MysteryDungeonRank::Diamond => "<:badgediamond:1272532683431481445>",
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
    /// Used for Struggle
    StrengthOrSpecial,
    /// Used for Copycat
    Copy,
    /// Used for Fixed Damage moves
    Rank,
    // Used for Return
    StrengthPlusRank,
    // Used for Frustration
    StrengthMinusRank,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SocialStat {
    Tough,
    Cool,
    Beauty,
    Clever,
    Cute,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, ChoiceParameter)]
pub enum Gender {
    Genderless = 0,
    Male = 1,
    Female = 2,
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

impl Gender {
    pub fn from_phenotype(value: i64) -> Gender {
        match value {
            1 => Gender::Male,
            2 => Gender::Female,
            _ => Gender::Genderless,
        }
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
            Stat::Rank => "Rank",
            Stat::StrengthPlusRank => "Strength + Rank",
            Stat::StrengthMinusRank => "Strength - Rank",
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
    MissingHappiness,
}

impl fmt::Display for HappinessDamageModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            HappinessDamageModifier::Happiness => "Happiness",
            HappinessDamageModifier::MissingHappiness => "Missing Happiness",
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
    Nine,
}

impl PokemonGeneration {
    pub fn has_animated_sprite(&self) -> bool {
        self <= &PokemonGeneration::Five
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    Deserialize,
    PartialOrd,
    PartialEq,
    poise::ChoiceParameter,
    sqlx::Type,
    FromRepr,
)]
#[repr(i64)]
pub enum QuestParticipantSelectionMechanism {
    #[name = "First Come First Serve"]
    FirstComeFirstServe = 1,
    #[name = "GM Picks"]
    GMPicks = 2,
    Random = 3,
}
