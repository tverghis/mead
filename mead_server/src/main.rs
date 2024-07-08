use axum::{routing::get, Json, Router};
use libbpf_rs::query::{ProgInfoIter, ProgInfoQueryOptions};
use schema::responses::ProgInfoResponse;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ping", get(ping_handler))
        .route("/prog_info", get(get_ebpf_programs));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on port 3000!");
    axum::serve(listener, app).await.unwrap();
}

async fn ping_handler() -> &'static str {
    "Pong!"
}

async fn get_ebpf_programs() -> Json<Vec<ProgInfoResponse>> {
    let query_opts = ProgInfoQueryOptions::default()
        .include_map_ids(true)
        .include_xlated_prog_insns(true);
    Json(
        ProgInfoIter::with_query_opts(query_opts)
            .map(ProgInfoResponse::from)
            .collect(),
    )
}
