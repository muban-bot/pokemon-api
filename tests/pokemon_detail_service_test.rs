#[cfg(test)]
mod tests {
    use pokemon::pokedex::service::PokemonDetailService;
    use pokemon::pokedex::client::PokeApiClient;
    use pokemon::translation::client::ShakespeareApiClient;

    #[tokio::test]
    async fn test_can_fetch_pokemon_case_insensitive() {
        let pokemon_client = PokeApiClient::new();

        // given some pokemon with an upper case name
        let detail = pokemon_client.get_pokemon_species_description("ChaRizArd".to_string()).await;

        // we should get a description
        let desc = detail.unwrap_or("no description".to_string());
        assert_eq!(desc, "Spits fire that is hot enough to melt boulders. Known to cause forest fires unintentionally.")
    }

    #[tokio::test]
    async fn test_can_fetch_pokemon_with_space_in_name() {
        let pokemon_client = PokeApiClient::new();

        // given some pokemon with a name that has spaces
        let detail = pokemon_client.get_pokemon_species_description("Mr Mime".to_string()).await;

        // we should get a description
        let desc = detail.unwrap_or("no description".to_string());
        assert_eq!(desc, "If interrupted while it is miming, it will slap around the offender with its broad hands.")
    }

    #[tokio::test]
    async fn test_can_fetch_pokemon_desc_with_stripped_chars() {
        let pokemon_client = PokeApiClient::new();
        let detail = pokemon_client.get_pokemon_species_description("Charizard".to_string()).await;
        let desc = detail.unwrap_or("no description".to_string());
        assert_eq!(desc, "Spits fire that is hot enough to melt boulders. Known to cause forest fires unintentionally.")
    }

    #[tokio::test]
    async fn test_can_fetch_pokemon_desc_and_translate() {
        let pokemon_client = PokeApiClient::new();
        let translation_client = ShakespeareApiClient::new();

        let service = PokemonDetailService::new(pokemon_client, translation_client);

        let detail = service.get_pokemon_description(String::from("Charizard")).await;
        let pokemon_detail = detail.unwrap();
        assert_eq!(pokemon_detail.description, "Spits fire yond is hot enow to melt boulders. Known to cause forest fires unintentionally.")
    }
}