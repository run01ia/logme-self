use std::sync::Arc;

use axum::{Router, routing::get, response::Html};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod report;
use report::ReportRepository;

mod web;
use web::{daily_report, latest_report};

#[derive(Clone)]
pub struct AppState {
    pub reports: Arc<ReportRepository>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
                "logme_self=debug,tower_http=debug",
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let reports = Arc::new(ReportRepository::new("./reports"));

    let app = Router::new()
        .route("/", get(handler))
        .route("/reports/daily/latest", get(latest_report))
        .route("/reports/daily/{date}", get(daily_report))
        .with_state(AppState { reports });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.expect("Server Error!");
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
