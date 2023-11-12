use axum::{
    response::sse::{Event, Sse},
    routing::get,
    Router,
    Json,
    extract::Path,
};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

use axum::response::IntoResponse;
use std::convert::Infallible;
use rand::Rng;
use tokio::time::{self, Duration};
use tokio_stream::StreamExt;

async fn roll_dice_handler(Path((num, sides)): Path<(u32, u32)>) -> impl IntoResponse {
    let mut rng = rand::thread_rng();
    let rolls: Vec<u32> = (0..num).map(|_| rng.gen_range(1..=sides)).collect();
    Json(rolls)
}

async fn sse_handler() -> impl IntoResponse {
    let stream = tokio_stream::wrappers::IntervalStream::new(time::interval(Duration::from_secs(1)))
        .map(|_| {
            let mut rng = rand::thread_rng();
            let random_number: u32 = rng.gen();
            Ok::<_, Infallible>(Event::default().data(random_number.to_string()))
        });
    Sse::new(stream)
}

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([45,79,139,237], 6014));
    println!("Hello from Axum. Server running at http://{}", addr);
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/sse", get(sse_handler))
        .route("/roll/:num/:sides", get(roll_dice_handler));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    println!("Ran server.");
}

