use crate::data::parser::helpers;

use crate::data::pokerole_data::raw_pokemon::RawPokerolePokemon;
use crate::data::pokerole_data::raw_ability::RawPokeroleAbility;
use crate::data::pokerole_data::raw_item::RawPokeroleItem;
use crate::data::pokerole_data::raw_move::RawPokeroleMove;
use crate::data::pokerole_data::raw_nature::RawPokeroleNature;

pub struct PokeroleParseResult {
    pub abilities: Vec<RawPokeroleAbility>,
    pub items: Vec<RawPokeroleItem>,
    pub moves: Vec<RawPokeroleMove>,
    pub natures: Vec<RawPokeroleNature>,
    pub pokemon: Vec<RawPokerolePokemon>,
}

pub fn parse(repo_path: &str) -> PokeroleParseResult {
    let mut items: Vec<RawPokeroleItem> = helpers::parse_directory(repo_path.to_owned() + "Version20/Items");
    items.extend(helpers::parse_directory(repo_path.to_owned() + "Homebrew/Items"));

    PokeroleParseResult {
        abilities: helpers::parse_directory(repo_path.to_owned() + "Version20/Abilities"),
        items,
        moves: helpers::parse_directory(repo_path.to_owned() + "Version20/Moves"),
        natures: helpers::parse_directory(repo_path.to_owned() + "Version20/Natures"),
        pokemon: helpers::parse_directory(repo_path.to_owned() + "Version20/Pokedex"),
    }
}
