use crate::entities::PokemonDescriptionResponse;

pub async fn handle_get_pokemon_detail(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&PokemonDescriptionResponse {
        name: name.to_string(),
        description: "".to_string(),
    }))
}