use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!("pokemon" / String)
        .map(|name| format!("Hello {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 5000))
        .await;
}
