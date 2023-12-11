use axum::{
    response::sse::{Event, Sse},
    routing::{get, post},
    Router,
    Json,
    extract::Path,
};
use hyper::{Body, Request, Response};
use std::collections::HashMap;
use http::StatusCode;

use axum::response::IntoResponse;
use std::convert::Infallible;
use rand::Rng;
use tokio::time::{self, Duration};
use tokio_stream::StreamExt;

async fn roll_dice_handler(Path((num, sides)): Path<(u32, u32)>) -> Result<Json<Vec<u32>>, (StatusCode, &'static str)> {
    if sides > 100 {
        return Err((StatusCode::BAD_REQUEST, "Error: Dice can have a maximum of 100 sides."));
    }
    let mut rng = rand::thread_rng();
    let rolls: Vec<u32> = (0..num).map(|_| rng.gen_range(1..=sides)).collect();
    Ok(Json(rolls))
}

use maplit::hashmap;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn send_handler(Json(payload): Json<HashMap<String, String>>) -> Json<HashMap<&'static str, bool>> {
    println!("{:?}", payload);
    Json(hashmap!{"ok" => true})
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
        .route("/roll/:num/:sides", get(roll_dice_handler))
        .route("/send", post(send_handler))
        .layer(middleware::from_fn(logging_middleware));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    println!("Ran server.");
}

use axum::middleware::{self, Next};


async fn logging_middleware(req: Request<Body>, next: Next<Body>) -> Result<Response<Body>, Infallible> {
    println!("Path: {}", req.uri().path());
    println!("Headers: {:?}", req.headers());
    let response = next.run(req).await;
    let (parts, body) = response.into_parts();
    let body = Body::from(hyper::body::to_bytes(body).await.unwrap());
    Ok(Response::from_parts(parts, body))
}
