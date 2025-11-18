mod error;
mod prelude;
mod state;
use axum::{
    extract::State,
    http::{
        header::{ACCEPT, CONTENT_TYPE},
        Method,
    },
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use lettre::AsyncTransport;
use prelude::*;
use serde::Deserialize;
use tower_http::{cors, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use tracing::*;

#[tokio::main]
async fn main() {
    // NOTE: Loading environment variables
    //dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=info,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port = std::env::var("PORT").unwrap_or(String::from("2020"));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    debug!("running on 0.0.0.0:{port}");

    axum::serve(listener, app()).await.unwrap();
}

#[derive(Debug, Clone, Deserialize)]
struct Email {
    from: String,
    subject: String,
    body: String,
}

#[tracing::instrument(skip(state))]
async fn email(State(state): State<AppState>, Json(json): Json<Email>) -> Result<(), AppError> {
    info!("sending message");
    let message = lettre::message::MessageBuilder::new()
        .from("johanjyyim@gmail.com".parse().unwrap())
        .to("johanjyyim@gmail.com".parse().unwrap())
        .subject(json.subject)
        .body(format!("{} from {}", json.body, json.from))
        .unwrap();
    state.mailer.send(message).await?;
    Ok(())
}

pub fn app() -> Router {
    let cors = cors::CorsLayer::new()
        .allow_origin(["http://localhost:5173".parse().unwrap()])
        .allow_credentials(true)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::POST,
        ])
        .allow_headers([ACCEPT, CONTENT_TYPE]);

    let state = AppState::new();

    //let api = Router::<AppState>::new()
    //    //.merge(admin_routes)
    //    //.merge(user_routes)
    //    // .route("/refresh", post(refresh))
    //    //.with_state(state)

    Router::new()
        .route("/", get(async || "backend is running"))
        .route("/email", post(email))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors)

    //.nest("/user", user_pages)
    //.nest("/api", api)
    //.fallback_service(public_pages)
}
