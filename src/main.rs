use axum::{routing, Json, Router};
use icalendar::{Calendar, CalendarComponent, Component};
use reqwest::{get, StatusCode};
use serde::Serialize;
use tower_http::cors::{CorsLayer, Any};

#[derive(Serialize, Debug)]
struct ReservedDates {
    start_date: String,
    end_date: String,
}

async fn get_apartment_ics() -> (StatusCode, Json<Vec<ReservedDates>>) {
    let mut dates: Vec<ReservedDates> = Vec::new();
    let response = get("")
        .await
        .unwrap()
        .text()
        .await;

    let parsed_calendar: Calendar = response.unwrap().parse().unwrap();

    for component in &parsed_calendar.components {
        if let CalendarComponent::Event(event) = component {
            dates.push(ReservedDates {
                start_date: event.get_start().unwrap().date_naive().to_string(),
                end_date: event.get_end().unwrap().date_naive().to_string(),
            });
        }
    }
    (StatusCode::OK, Json(dates))
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let app = Router::new()
        .route("/", routing::get(|| async { "Hello world" }))
        .route("/get_dates", routing::get(get_apartment_ics))
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Starting up server on port 3000");
    axum::serve(listener, app).await.unwrap();
}
