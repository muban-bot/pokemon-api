use crate::pokedex::client::{PokeApiClient};
use crate::translation::client::ShakespeareApiClient;

#[derive(Clone)]
pub struct PokemonDetailService {
    pokemon_client: PokeApiClient,
    translation_client: ShakespeareApiClient,
}

pub struct PokemonDetail {
    pub description: String,
}

impl PokemonDetailService {
    pub fn new(pokemon_client: PokeApiClient, translation_client: ShakespeareApiClient) -> PokemonDetailService {
        PokemonDetailService {
            pokemon_client,
            translation_client,
        }
    }

    pub async fn get_pokemon_description(&self, pokemon_name: String) -> Option<PokemonDetail> {
        let details = self.pokemon_client.get_pokemon_species_description(pokemon_name)
            .await
            .unwrap();

        let translation_resp = self.translation_client.translate_string(details)
            .await;

        let translation = translation_resp.ok()?;
        Some(PokemonDetail {
            description: translation,
        })
    }
}