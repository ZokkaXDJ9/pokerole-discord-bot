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

#[derive(Debug, Clone, Copy)]
pub enum HappinessDamageModifier {
    Happiness,
    MissingHappiness
}
