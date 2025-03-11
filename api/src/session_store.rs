// Session store for online play.
// Keep track of which sockets are connected and are players in each room.

use dashmap::DashMap;
use serde::Serialize;
use socketioxide::socket::Sid;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Session {
    pub uuid: Uuid,
    pub username: String,
    pub room: Option<String>,
    pub color: Option<String>,
}

impl Session {
    pub fn new(username: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            username,
            room: None,
            color: None,
        }
    }

    pub fn set_game(&mut self, room: String, color: String) {
        self.room = Some(room);
        self.color = Some(color);
    }
}

#[derive(Clone, Default)]
pub struct SessionStore(Arc<RwLock<HashMap<Uuid, Arc<Session>>>>);

impl SessionStore {
    pub fn get(&self, uuid: &Uuid) -> Option<Arc<Session>> {
        self.0.read().unwrap().get(uuid).cloned()
    }

    pub fn add(&self, session: Arc<Session>) {
        self.0.write().unwrap().insert(session.uuid, session);
    }

    pub fn remove(&self, session: &Uuid) {
        self.0.write().unwrap().remove(session);
    }
}

pub struct GameSession {
    pub roomcode: String,
    pub white: Sid,
    pub black: Sid,
}

pub struct GameSessionStore(Arc<DashMap<String, GameSession>>); // Arc<GameSession>?
