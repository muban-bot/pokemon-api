use warp::Filter;

mod pokemon_handler;
mod pokedex;
mod translation;

use crate::pokemon_handler::handle_get_pokemon_detail;
use crate::pokedex::client::PokeApiClient;
use crate::pokedex::service::PokemonDetailService;
use crate::translation::client::ShakespeareApiClient;

#[tokio::main]
async fn main() {
    let pokemon_client = PokeApiClient::new();
    let translation_client = ShakespeareApiClient::new();
    let pokemon_service = PokemonDetailService::new(pokemon_client, translation_client);

    let pokemon_service_filter =
        warp::any().map(move || pokemon_service.clone());

    let routes = warp::path("pokemon")
        .and(warp::get())
        .and(warp::path::param::<String>())
        .and(pokemon_service_filter.clone())
        .and_then(handle_get_pokemon_detail);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 5000))
        .await;
}
