use axum::extract::{Path, State, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use futures_util::{sink::SinkExt, stream::StreamExt};
use tracing::log;
use crate::AppState;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Path(room_id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    log::info!("New WebSocket connection attempt for room: {}", room_id);
    ws.on_upgrade(move |socket| handle_socket(socket, room_id, state))
}

async fn handle_socket(socket: WebSocket, room_id: String, state: AppState) {
    log::info!("WebSocket connection established for room: {}", room_id);
    let (mut sender, mut receiver) = socket.split();
    let game_room = state.cache.get_or_create_channel_and_game(&room_id).await;
    let mut rx = game_room.tx.subscribe();

    // Send initial gamestate
    let game_state_message = Message::Text(game_room.get_state_copy().await);
    let _ = sender.send(game_state_message).await;

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg_to_send) = rx.recv().await {
            if sender.send(Message::Text(msg_to_send)).await.is_err() {
                log::info!("Failed to send message to WebSocket in room");
                break
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg_received)) = receiver.next().await {
            match msg_received {
                Message::Text(text) => {
                    log::info!( "Received message: {}", text);
                    game_room.make_move(&text).await;
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
            log::info!("Send task finished for room {}, aborting receive task.", room_id);
            recv_task.abort();
        },
        _ = (&mut recv_task) => {
            log::info!("Receive task finished for room {}, aborting send task.", room_id);
            send_task.abort();
        },
    }
}