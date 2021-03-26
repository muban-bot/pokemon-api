use warp::Filter;

mod pokemon_handler;
mod entities;

use pokemon_handler::{handle_get_pokemon_detail};

#[tokio::main]
async fn main() {
    let routes = warp::path("pokemon")
        .and(warp::get())
        .and(warp::path::param::<String>())
        .and_then(handle_get_pokemon_detail);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 5000))
        .await;
}
