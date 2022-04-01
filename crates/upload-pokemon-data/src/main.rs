use color_eyre::eyre;
use serde::{de, Deserialize};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let config = aws_config::from_env().load().await;

    dbg!(config.region());

    let pokemon = csv::Reader::from_path(
        "./crates/upload-pokemon-data/pokemon.csv",
    )?
    .deserialize()
    .collect::<Result<Vec<PokemonCsv>, csv::Error>>()?;

    dbg!(&pokemon[0]);

    Ok(())
}

#[derive(Debug, Deserialize, Clone)]
struct PokemonCsv {
    name: String,
    pokedex_id: u16,
    #[serde(deserialize_with = "from_comma_separated")]
    abilities: Vec<String>,
    #[serde(deserialize_with = "from_comma_separated")]
    typing: Vec<String>,
    hp: u8,
    attack: u8,
    defense: u8,
    special_attack: u8,
    special_defense: u8,
    speed: u8,
    height: u16,
    weight: u16,
    generation: u8,
    female_rate: Option<f32>,
    #[serde(deserialize_with = "from_capital_bool")]
    genderless: bool,
    #[serde(
        rename(deserialize = "legendary/mythical"),
        deserialize_with = "from_capital_bool"
    )]
    is_legendary_or_mythical: bool,
    #[serde(deserialize_with = "from_capital_bool")]
    is_default: bool,
    #[serde(deserialize_with = "from_capital_bool")]
    forms_switchable: bool,
    base_experience: u16,
    capture_rate: u8,
    #[serde(deserialize_with = "from_comma_separated")]
    egg_groups: Vec<String>,
    base_happiness: u8,
    evolves_from: Option<String>,
    primary_color: String,
    number_pokemon_with_typing: f32,
    normal_attack_effectiveness: f32,
    fire_attack_effectiveness: f32,
    water_attack_effectiveness: f32,
    electric_attack_effectiveness: f32,
    grass_attack_effectiveness: f32,
    ice_attack_effectiveness: f32,
    fighting_attack_effectiveness: f32,
    poison_attack_effectiveness: f32,
    ground_attack_effectiveness: f32,
    fly_attack_effectiveness: f32,
    psychic_attack_effectiveness: f32,
    bug_attack_effectiveness: f32,
    rock_attack_effectiveness: f32,
    ghost_attack_effectiveness: f32,
    dragon_attack_effectiveness: f32,
    dark_attack_effectiveness: f32,
    steel_attack_effectiveness: f32,
    fairy_attack_effectiveness: f32,
}

fn from_capital_bool<'de, D>(
    deserializer: D,
) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str =
        de::Deserialize::deserialize(deserializer)?;

    match s {
        "True" => Ok(true),
        "False" => Ok(false),
        _ => Err(de::Error::custom("not a boolean!")),
    }
}

fn from_comma_separated<'de, D>(
    deserializer: D,
) -> Result<Vec<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str =
        de::Deserialize::deserialize(deserializer)?;

    Ok(s.split(", ")
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .collect())
}
