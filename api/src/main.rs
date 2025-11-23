mod configuration;
mod game_dto;
mod handlers;
mod socket_handlers;
mod socket_handler;
mod game_room;
mod handler;
mod dtos;
mod game_store;

use std::env;
use crate::configuration::ApplicationSettings;
use axum::response::Redirect;
use axum::{
    routing::{get},
    Router,
};
use env_logger::Env;
use std::net::SocketAddr;
use axum::routing::post;
use sqlx::postgres::PgPoolOptions;
use tower_http::services::fs::ServeFile;
use tower_http::services::ServeDir;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use tracing::log;
use persistence::elo_stealo_postgres::EloStealoPostgresStore;
use crate::game_store::GameStore;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("api=info")).init();

    let settings = ApplicationSettings::load()
        .map_err(|e| log::error!("Error while loading settings: {}", e))
        .unwrap();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(
        |_| "postgres://postgres:postgres@localhost:5432/EloStealo".into());
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(database_url.as_str())
        .await
        .expect("Could not connect to postgres");

    let repository = EloStealoPostgresStore::new(pool).await.expect("Failed to create EloStealoPostgresStore");
    let game_store = GameStore::new(1000, repository);
    let state = AppState { game_store };

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnSessionEnd);

    let client = ServeDir::new("./client/dist").fallback(ServeFile::new("index.html"));

    let app = Router::new()
        .nest_service("/", client)
        .route("/online", get(|| async { Redirect::permanent("/") }))
        .route("/about", get(|| async { Redirect::permanent("/") }))
        .route("/api/startgame", post(handler::start_local_game))
        .route("/api/rules", get(handler::stealo_rules))
        .route("/ws/:room_id", get(socket_handler::websocket_handler))
        .layer(session_layer)
        .with_state(state);

    let addr = SocketAddr::from((settings.host, settings.port));
    log::info!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

#[derive(Clone)]
struct AppState {
    game_store: GameStore,
}
