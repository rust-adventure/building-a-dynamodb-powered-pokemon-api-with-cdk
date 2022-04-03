use aws_sdk_dynamodb::{
    model::{AttributeValue, PutRequest, WriteRequest},
    Client,
};
use color_eyre::eyre;
use inflector::Inflector;
use serde::{de, Deserialize};
use std::{collections::HashMap, env};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let table_name = env::var("TABLE_NAME")?;

    let config = aws_config::from_env().load().await;

    let client = Client::new(&config);

    let pokemon = csv::Reader::from_path(
        "./crates/upload-pokemon-data/pokemon.csv",
    )?
    .deserialize()
    .collect::<Result<Vec<PokemonCsv>, csv::Error>>()?;

    for poke_chunk in pokemon.chunks(25) {
        let batch = poke_chunk
            .iter()
            .cloned()
            .map(|pokemon| {
                let mut map = HashMap::new();
                map.insert(
                    "name".to_string(),
                    AttributeValue::S(pokemon.name.clone()),
                );
                map.insert(
                    "pokedex_id".to_string(),
                    AttributeValue::N(
                        pokemon.pokedex_id.to_string(),
                    ),
                );
                map.insert(
                    "abilities".to_string(),
                    AttributeValue::L(
                        pokemon
                            .abilities
                            .into_iter()
                            .map(AttributeValue::S)
                            .collect(),
                    ),
                );
                map.insert(
                    "typing".to_string(),
                    AttributeValue::L(
                        pokemon
                            .typing
                            .into_iter()
                            .map(AttributeValue::S)
                            .collect(),
                    ),
                );
                map.insert(
                    "health_points".to_string(),
                    AttributeValue::N(
                        pokemon.hp.to_string(),
                    ),
                );
                map.insert(
                    "attack".to_string(),
                    AttributeValue::N(
                        pokemon.attack.to_string(),
                    ),
                );
                map.insert(
                    "defense".to_string(),
                    AttributeValue::N(
                        pokemon.defense.to_string(),
                    ),
                );
                map.insert(
                    "special_attack".to_string(),
                    AttributeValue::N(
                        pokemon.special_attack.to_string(),
                    ),
                );
                map.insert(
                    "special_defense".to_string(),
                    AttributeValue::N(
                        pokemon.special_defense.to_string(),
                    ),
                );
                map.insert(
                    "speed".to_string(),
                    AttributeValue::N(
                        pokemon.speed.to_string(),
                    ),
                );
                map.insert(
                    "height".to_string(),
                    AttributeValue::N(
                        pokemon.height.to_string(),
                    ),
                );
                map.insert(
                    "weight".to_string(),
                    AttributeValue::N(
                        pokemon.weight.to_string(),
                    ),
                );
                map.insert(
                    "generation".to_string(),
                    AttributeValue::N(
                        pokemon.generation.to_string(),
                    ),
                );
                if let Some(rate) = pokemon.female_rate {
                    map.insert(
                        "female_rate".to_string(),
                        AttributeValue::N(rate.to_string()),
                    );
                }

                map.insert(
                    "genderless".to_string(),
                    AttributeValue::Bool(
                        pokemon.genderless,
                    ),
                );
                map.insert(
                    "is_legendary_or_mythical".to_string(),
                    AttributeValue::Bool(
                        pokemon.is_legendary_or_mythical,
                    ),
                );
                map.insert(
                    "is_default".to_string(),
                    AttributeValue::Bool(
                        pokemon.is_default,
                    ),
                );
                map.insert(
                    "forms_switchable".to_string(),
                    AttributeValue::Bool(
                        pokemon.forms_switchable,
                    ),
                );
                map.insert(
                    "base_experience".to_string(),
                    AttributeValue::N(
                        pokemon.base_experience.to_string(),
                    ),
                );
                map.insert(
                    "capture_rate".to_string(),
                    AttributeValue::N(
                        pokemon.capture_rate.to_string(),
                    ),
                );
                map.insert(
                    "egg_groups".to_string(),
                    AttributeValue::L(
                        pokemon
                            .egg_groups
                            .into_iter()
                            .map(AttributeValue::S)
                            .collect(),
                    ),
                );
                map.insert(
                    "base_happiness".to_string(),
                    AttributeValue::N(
                        pokemon.base_happiness.to_string(),
                    ),
                );
                if let Some(evolves_from) =
                    pokemon.evolves_from
                {
                    map.insert(
                        "evolves_from".to_string(),
                        AttributeValue::S(evolves_from),
                    );
                }

                map.insert(
                    "primary_color".to_string(),
                    AttributeValue::S(
                        pokemon.primary_color,
                    ),
                );
                map.insert(
                    "number_pokemon_with_typing"
                        .to_string(),
                    AttributeValue::N(
                        pokemon
                            .number_pokemon_with_typing
                            .to_string(),
                    ),
                );
                map.insert(
                    "normal_attack_effectiveness"
                        .to_string(),
                    AttributeValue::N(
                        pokemon
                            .normal_attack_effectiveness
                            .to_string(),
                    ),
                );
                map.insert(
                    "fire_attack_effectiveness".to_string(),
                    AttributeValue::N(
                        pokemon
                            .fire_attack_effectiveness
                            .to_string(),
                    ),
                );
                map.insert(
                    "water_attack_effectiveness"
                        .to_string(),
                    AttributeValue::N(
                        pokemon
                            .water_attack_effectiveness
                            .to_string(),
                    ),
                );
                map.insert(
                    "electric_attack_effectiveness"
                        .to_string(),
                    AttributeValue::N(
                        pokemon
                            .electric_attack_effectiveness
                            .to_string(),
                    ),
                );
                map.insert(
                    "grass_attack_effectiveness"
                        .to_string(),
                    AttributeValue::N(
                        pokemon
                            .grass_attack_effectiveness
                            .to_string(),
                    ),
                );
                map.insert(
                    "ice_attack_effectiveness".to_string(),
                    AttributeValue::N(
                        pokemon
                            .ice_attack_effectiveness
                            .to_string(),
                    ),
                );
                map.insert(
                    "fighting_attack_effectiveness"
                        .to_string(),
                    AttributeValue::N(
                        pokemon
                            .fighting_attack_effectiveness
                            .to_string(),
                    ),
                );
                map.insert(
                    "poison_attack_effectiveness"
                        .to_string(),
                    AttributeValue::N(
                        pokemon
                            .poison_attack_effectiveness
                            .to_string(),
                    ),
                );
                map.insert(
                    "ground_attack_effectiveness"
                        .to_string(),
                    AttributeValue::N(
                        pokemon
                            .ground_attack_effectiveness
                            .to_string(),
                    ),
                );
                map.insert(
                    "fly_attack_effectiveness".to_string(),
                    AttributeValue::N(
                        pokemon
                            .fly_attack_effectiveness
                            .to_string(),
                    ),
                );
                map.insert(
                    "psychic_attack_effectiveness"
                        .to_string(),
                    AttributeValue::N(
                        pokemon
                            .psychic_attack_effectiveness
                            .to_string(),
                    ),
                );
                map.insert(
                    "bug_attack_effectiveness".to_string(),
                    AttributeValue::N(
                        pokemon
                            .bug_attack_effectiveness
                            .to_string(),
                    ),
                );
                map.insert(
                    "rock_attack_effectiveness".to_string(),
                    AttributeValue::N(
                        pokemon
                            .rock_attack_effectiveness
                            .to_string(),
                    ),
                );
                map.insert(
                    "ghost_attack_effectiveness"
                        .to_string(),
                    AttributeValue::N(
                        pokemon
                            .ghost_attack_effectiveness
                            .to_string(),
                    ),
                );

                map.insert(
                    "dragon_attack_effectiveness"
                        .to_string(),
                    AttributeValue::N(
                        pokemon
                            .dragon_attack_effectiveness
                            .to_string(),
                    ),
                );
                map.insert(
                    "dark_attack_effectiveness".to_string(),
                    AttributeValue::N(
                        pokemon
                            .dark_attack_effectiveness
                            .to_string(),
                    ),
                );
                map.insert(
                    "steel_attack_effectiveness"
                        .to_string(),
                    AttributeValue::N(
                        pokemon
                            .steel_attack_effectiveness
                            .to_string(),
                    ),
                );
                map.insert(
                    "fairy_attack_effectiveness"
                        .to_string(),
                    AttributeValue::N(
                        pokemon
                            .fairy_attack_effectiveness
                            .to_string(),
                    ),
                );

                map.insert(
                    "pk".to_string(),
                    AttributeValue::S(
                        pokemon.name.to_kebab_case(),
                    ),
                );

                WriteRequest::builder()
                    .put_request(
                        PutRequest::builder()
                            .set_item(Some(map))
                            .build(),
                    )
                    .build()
            })
            .collect::<Vec<WriteRequest>>();
        let sent = client
            .batch_write_item()
            .request_items(&table_name, batch)
            .send()
            .await?;
        if let Some(items) = sent.unprocessed_items {
            if !items.is_empty() {
                panic!("items didn't make it");
            }
        }
    }

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
