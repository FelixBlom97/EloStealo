use uuid::Uuid;
use axum::{
    extract::Path
};
use axum::extract::{State, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use futures::{SinkExt, StreamExt};
use tokio::sync::broadcast;
use tracing::log;
use crate::AppState;

// Handler for WebSocket upgrade requests
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(game_id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    log::info!("New WebSocket connection attempt for game: {}", game_id);
    ws.on_upgrade(move |socket| handle_socket(socket, state, game_id))
}

async fn handle_socket(socket: WebSocket, state: AppState, game_id: String) {
    let tx = state.channels
        .entry(game_id.clone())
        .or_insert_with(|| {
            log::info!("Creating new broadcast channel for game");
            let (tx, _rx) = broadcast::channel::<String>(100); // Channel capacity 100 messages
            tx
        })
        .value()
        .clone(); // Clone the sender

    // Subscribe to the broadcast channel *before* splitting the socket
    let mut rx = tx.subscribe();

    // Split the WebSocket into sender and receiver
    let (mut sender, mut receiver) = socket.split();

    // Task to forward broadcast messages to the client's WebSocket sender
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // Send the message received from the broadcast channel to the WebSocket client
            if sender.send(Message::Text(msg)).await.is_err() {
                // Client disconnected or error sending
                break;
            }
        }
        log::info!("Send task finished (client disconnected or channel closed)");
    });

    // Task to handle messages received from the client
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    // Broadcast the received message to all subscribers of this game's channel
                    log::info!("Received message: {}", text);
                    // Here you might want to add sender info, validate, etc.
                    // For simplicity, we just broadcast the raw text.
                    // The sender will also receive their own message back via the broadcast mechanism.
                    if let Err(e) = tx.send(text) {
                        log::error!("Failed to broadcast message: {}", e);
                        // If sending fails, it might mean no active subscribers are left.
                    }
                }
                Message::Close(_) => {
                    log::info!("Received close message");
                    break; // Exit loop on WebSocket close message
                }
                Message::Ping(_) | Message::Pong(_) | Message::Binary(_) => {
                    // Ignore ping/pong/binary for this simple example
                }
            }
        }
        log::info!("Receive task finished (client disconnected or sent close)");
    });

    // Wait for either task to complete (meaning the connection is closing)
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort(); // If send task finishes (e.g., error), abort recv task
            log::info!("Send task completed first.");
        },
        _ = (&mut recv_task) => {
            send_task.abort(); // If recv task finishes (e.g., client disconnected), abort send task
            log::info!("Receive task completed first.");
        },
    }

    log::info!("WebSocket connection closed.");
}