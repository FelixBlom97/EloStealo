use std::sync::Arc;
use axum::extract::{Path, State, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use futures_util::{sink::SinkExt, stream::StreamExt};
use tracing::log;
use crate::AppState;

async fn websocket_handler(
    ws: WebSocketUpgrade,
    Path(room_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    log::info!("New WebSocket connection attempt for room: {}", room_id);
    ws.on_upgrade(move |socket| handle_socket(socket, room_id, state))
}

async fn handle_socket(socket: WebSocket, room_id: String, state: Arc<AppState>) {
    log::info!("WebSocket connection established for room: {}", room_id);
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = state.cache.get_or_create_transceivers(&room_id).await;

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg_to_send) = rx.recv().await {
            if sender.send(Message::Text(msg_to_send)).await.is_err() {
                println!(
                    "Failed to send message to WebSocket in room"
                );
                break;            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg_received)) = receiver.next().await {
            match msg_received {
                Message::Text(text) => {
                    println!( "Received message: {}", text);
                    if tx.send(text).is_err() {
                        println!(
                            "No active subscribers in room.",
                        );
                    }
                }
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => {
            println!("Send task finished for room {}, aborting receive task.", room_id);
            recv_task.abort();
        },
        _ = (&mut recv_task) => {
            println!("Receive task finished for room {}, aborting send task.", room_id);
            send_task.abort();
        },
    }
}