use axum::extract::{Path, State, WebSocketUpgrade};
use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use axum::http::StatusCode;
use axum::response::{Response};
use futures_util::{sink::SinkExt, stream::StreamExt, TryFutureExt};
use tower_sessions::Session;
use tracing::log;
use uuid::Uuid;
use persistence::game_id::GameId;
use crate::AppState;
use crate::game_room::GameRoom;
use crate::handler::get_or_create_user_uuid;

pub async fn websocket_handler(
    session: Session,
    ws: WebSocketUpgrade,
    Path(room_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Response, StatusCode> {
    log::info!("New WebSocket connection attempt for room: {}", room_id);

    // Ensure the room exists before upgrading the websocket.
    let room_id = GameId::try_from(room_id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let game_room = state.game_store.try_get_game(room_id).await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let user_uuid: Uuid = get_or_create_user_uuid(session)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .await?;

    Ok(ws.on_upgrade(move |socket| handle_socket(socket, game_room, state, user_uuid)))
}

async fn handle_socket(socket: WebSocket, game_room: GameRoom, _state: AppState, user_uuid: Uuid) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = game_room.tx.subscribe();

    // Send initial gamestate
    let game_state_message = Message::Text(Utf8Bytes::from(game_room.get_game_state().await));
    let _ = sender.send(game_state_message).await;

    // Send game info
    let game_info_message = Message::Text(Utf8Bytes::from(game_room.get_game_info(&user_uuid).await));
    let _ = sender.send(game_info_message).await;

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg_to_send) = rx.recv().await {
            if sender.send(Message::Text(Utf8Bytes::from(msg_to_send))).await.is_err() {
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
                    game_room.make_move(text.to_string(), &user_uuid).await;
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
            log::info!("Send task finished for room aborting receive task.");
            recv_task.abort();
        },
        _ = (&mut recv_task) => {
            log::info!("Receive task finished for room aborting send task.");
            send_task.abort();
        },
    }
}