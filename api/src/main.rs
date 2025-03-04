mod configuration;
mod game_dto;
mod handlers;
mod session_store;
mod socket_handlers;

use crate::configuration::ApplicationSettings;
use crate::session_store::SessionStore;
use axum::response::Redirect;
use axum::{
    routing::{get, post},
    Router,
};
use env_logger::Env;
use mongodb::{options::ClientOptions, Client, Database};
use socketioxide::SocketIo;
use std::net::SocketAddr;
use tower_http::services::fs::ServeFile;
use tower_http::services::ServeDir;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tracing::log;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("api=info")).init();

    let settings = ApplicationSettings::load()
        .map_err(|e| log::error!("Error while loading settings: {}", e))
        .unwrap();

    let client_options = ClientOptions::parse("Imma change this to a containerized postgress db").await.unwrap();
    let client = Client::with_options(client_options).expect("Failed to connect to database.");
    let db = client.database("EloStealo");

    let state = AppState { database: db };

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);
    let (socket_layer, io) = SocketIo::builder()
        .with_state(state.clone())
        .with_state(SessionStore::default())
        .build_layer();
    io.ns("/api/socket", socket_handlers::on_connect);

    let client = ServeDir::new("./client/dist").fallback(ServeFile::new("index.html"));

    let app = Router::new()
        .nest_service("/", client)
        .route("/online", get(|| async { Redirect::permanent("/") }))
        .route("/about", get(|| async { Redirect::permanent("/") }))
        .route("/api/startgame", post(handlers::start_game))
        .route("/api/play", post(handlers::play))
        .route("/api/rules", get(handlers::stealo_rules))
        .route("/api/start_online", post(handlers::start_online))
        .route("/api/get_game_info", post(handlers::get_game_info))
        .route("/api/get_local_info", get(handlers::get_local_info))
        .layer(session_layer)
        .layer(socket_layer)
        .with_state(state);

    let addr = SocketAddr::from((settings.host, settings.port));
    log::info!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

// Pass the database connection as state to let axum/socketioxide handle the connection lifetimes
#[derive(Clone)]
struct AppState {
    database: Database,
}
