use std::str::FromStr;
use crate::game_dto::{create_game_dto, GameDTO, PlayOnlineMove, WaitingPlayer};
use crate::AppState;
use async_timers::PeriodicTimer;
use socketioxide::extract::{Data, SocketRef, State};
use std::time::Duration;
use serde_json::from_str;
use tracing::log;
use uuid::Uuid;

pub async fn on_connect(socket: SocketRef) {
    log::info!("Socket connected: {:?}", socket.id);
    // Check if it's a reconnection and the room code needs to be re-obtained from the client
    if socket.rooms().unwrap_or_default().is_empty() {
        let _ = socket.emit("connected", ());
    };

    socket.on(
        "reconnected",
        |socket: SocketRef, Data::<String>(room), state: State<AppState>| async move {
            log::info!("Socket {:?} reconnected to room {:?}", socket.id, &room);
            let _ = socket.join(room.clone());
            let load_chessgame = state.repository.get_game(Uuid::from_str(&room).unwrap()).await;
            match load_chessgame {
                Ok(chessgame) => {
                    let game_dto = create_game_dto(&chessgame);
                    let _ = socket.within(room).emit("sync", game_dto);
                }
                Err(_e) => {
                    let _ = socket.emit("error", ());
                }
            }
        },
    );

    // Split up create and join room
    socket.on(
        "create_room",
        |socket: SocketRef, Data::<WaitingPlayer>(player)| async move {
            let _ = socket.leave_all();
            let _ = socket.join(player.room);
            log::info!(
                "{:?} created room {:?}",
                socket.id,
                socket.rooms().unwrap_or_default()
            );
            socket.emit("join", ()).ok();
        },
    );

    socket.on(
        "join",
        |socket: SocketRef, Data::<WaitingPlayer>(player)| async move {
            let _ = socket.leave_all();
            let length = socket.within(player.room.clone()).sockets().unwrap().len();
            if length >= 2 {
                let _ = socket.emit("full", ()).ok();
            } else if length == 0 {
                socket.emit("room_not_found", ()).ok();
            } else {
                let _ = socket.join(player.room.clone());
                log::info!(
                    "{:?} joined room {:?}",
                    socket.id,
                    socket.rooms().unwrap_or_default()
                );
                socket
                    .within(player.room.clone())
                    .emit("create_game", player)
                    .ok();
            }
        },
    );

    socket.on(
        "leave",
        |socket: SocketRef, Data::<String>(room)| async move {
            log::info!("{:?} left room {:?}", room, socket.id);
            let _ = socket.leave([room]);
            socket.emit("leave", "").ok();
        },
    );

    socket.on(
        "start_game",
        |socket: SocketRef, Data::<GameDTO>(game)| async move {
            let room = socket.rooms().unwrap_or_default();
            log::info!("Game {:?} has started!", &room);
            let _ = socket.within(room).emit("start_game", game);
        },
    );

    socket.on(
        "move",
        |socket: SocketRef, Data::<PlayOnlineMove>(play_move), state: State<AppState>| async move {
            let room = play_move.roomcode;
            let load_chessgame = state.repository.get_game(Uuid::from_str(&room).unwrap()).await;
            match load_chessgame {
                Ok(mut chessgame) => {
                    chessgame.make_move(play_move.play_move, None);
                    match state.repository.update_game(Uuid::from_str(&room).unwrap(), &chessgame).await {
                        Ok(()) => {
                            let game_dto = create_game_dto(&chessgame);
                            let _ = socket.within(room).emit("sync", game_dto);
                        }
                        Err(_e) => {
                            socket.emit("error", ()).ok();
                        }
                    }
                }
                Err(_e) => {
                    socket.emit("error", ()).ok();
                }
            }
        },
    );

    socket.on_disconnect(|socket: SocketRef| {
        log::info!("{:?} disconnected", socket.id);
        let room = socket.rooms().unwrap_or_default();
        let _ = socket.within(room).emit("disconnected", ());
    });

    // Timing has to happen async, but on_disconnect cannot be async if we want to grab the room,
    // so the timer is triggered through the other socket in the room (if it exists).
    socket.on(
        "disconnect_timer",
        |socket: SocketRef, Data::<String>(room)| async move {
            log::info!("Player left room {:?}. Starting timer.", &room);
            if room == "" {
                return;
            }
            let mut timer = PeriodicTimer::started(Duration::from_secs(1));
            let mut count = 0;
            loop {
                count += 1;
                if count == 30 {
                    log::info!("Room {:?} abandoned", room.clone());
                    let _ = socket.within(room.clone()).emit("abandon", "").ok();
                    break;
                } else {
                    let length = socket.within(room.clone()).sockets().unwrap().len();
                    if length >= 2 {
                        log::info!("All players reconnected to {:?}", room);
                        return;
                    } else {
                        timer.tick().await;
                        continue;
                    }
                }
            }
        },
    );
}
