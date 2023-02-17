use axum::{
    routing::{get},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(index));
        // `POST /users` goes to `create_user`
        // .route("/users", post(create_user));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index( Json(payload): Json<GCDInput> ) -> ( StatusCode, Json<GCDResult> ) {
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type

    // insert your application logic here
    let mut d = payload.numbers[0];
    for m in &payload.numbers[1..] {
        d = gcd(d, *m);
    }
    
    let gcd_result = GCDResult {
        gcd: d,
        numbers: payload.numbers,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::OK, Json(gcd_result))
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn verify_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct GCDInput {
    numbers: Vec<u64>,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct GCDResult {
    gcd: u64,
    numbers: Vec<u64>,
}
