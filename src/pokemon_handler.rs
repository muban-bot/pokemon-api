use crate::pokedex::service::PokemonDetailService;
use crate::pokedex::entities::PokemonDescriptionResponse;

pub async fn handle_get_pokemon_detail(name: String, pokemon_detail_service: PokemonDetailService) -> Result<impl warp::Reply, warp::Rejection> {
    let pokemon_name_decoded = urldecode::decode(name);
    let detail = pokemon_detail_service.get_pokemon_description(pokemon_name_decoded.clone()).await;
    match detail {
        None => Err(warp::reject()),
        Some(details) => Ok(warp::reply::json(&PokemonDescriptionResponse {
            name: pokemon_name_decoded,
            description: details.description,
        }))
    }
}