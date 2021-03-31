use serde_json::Value;
use regex::Regex;

#[derive(Clone)]
pub struct PokeApiClient { pub endpoint: &'static str }

impl PokeApiClient {
    pub fn new() -> PokeApiClient {
        PokeApiClient {
            endpoint: "https://pokeapi.co/api/v2"
        }
    }

    pub async fn get_pokemon_species_description(&self, name: String) -> Result<String, Box<dyn std::error::Error>> {
        let sanitized_pokemon_name = name
            .to_ascii_lowercase()
            .replace(" ", "-");

        let uri = format!("{}/pokemon-species/{}", self.endpoint, sanitized_pokemon_name);
        println!("{:#?}", uri);

        let resp = reqwest::get(uri)
            .await?
            .text()
            .await
            .unwrap();

        let result: Value = serde_json::from_str(resp.as_str())?;

        let entries = result["flavor_text_entries"].as_array().unwrap();

        let description_entries: Vec<String> = entries.iter().filter(|&entry| {
            let language = &entry["language"]["name"];
            language == "en"
        }).map(|entry| {
            let text_entry = &entry["flavor_text"];
            text_entry.to_string()
        }).collect();

        if description_entries.is_empty() {
            return Err(format!("no descriptions for {}", name).into());
        }

        let first_entry = description_entries.get(0).unwrap();
        let re = Regex::new(r"\\n|\\f").unwrap();
        let desc_no_quotes = first_entry.get(1..first_entry.len() - 1).unwrap();
        return Ok(re.replace_all(desc_no_quotes, " ").to_string());
    }
}