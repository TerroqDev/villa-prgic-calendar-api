use axum::{extract::Path, routing, Extension, Json, Router};
use icalendar::{Calendar, CalendarComponent, Component};
use reqwest::{get, StatusCode};
use serde::Serialize;
use shuttle_runtime::SecretStore;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

mod middleware;

#[derive(Serialize, Debug, ToSchema)]
struct ReservedDates {
    start_date: String,
    end_date: String,
}

#[derive(OpenApi)]
#[openapi(paths(get_apartment_ics), components(schemas(ReservedDates)))]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/get_dates/{id}",
    params(
        ("id" = u8, Path, description = "Apartment ID")
    ),
    responses(
        (status = 200, description = "Reserved dates", body = [ReservedDates])
    )
)]
async fn get_apartment_ics(
    Path(id): Path<u8>,
    Extension(secret): Extension<SecretStore>,
) -> (StatusCode, Json<Vec<ReservedDates>>) {
    let apartment = format!("AIRBNB-A{}", id).to_string();
    let sec = secret.get(&apartment).unwrap();
    let mut dates: Vec<ReservedDates> = Vec::new();
    let response = get(sec).await.unwrap().text().await;

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

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
        .route("/get_dates/{id}", routing::get(get_apartment_ics))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(secrets))
                .layer(cors)
                .layer(axum::middleware::from_fn(middleware::api_key_auth)),
        );
    Ok(app.into())
}
