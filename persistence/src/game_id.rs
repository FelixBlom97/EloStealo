// Want a unique ID for games that is used everywhere.
// Therefore, it needs to:
//  1. Have enough randomness to avoid collisions.
//  2. Be (mostly) sequential for optimal database performance.
//  3. Be short for shorter urls and to allow people to share game codes without copy-pasting.
//
// Solution: take the last 41 bits in the number of milliseconds since Epoch amd subtract about 52 years.
// This is an ordered sequence for the next ~65 years.
// Add 23 bits of randomness to the end to avoid collisions when two games are created in the same
// millisecond, and save it into an u64.
// Convert to base58 to store it in a string of at most 11.

use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::clone::Clone;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, Hash)]
pub struct GameId(String);

impl GameId {
    pub fn new() -> Self {
        let milliseconds = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        let milliseconds_subtracted = milliseconds - (0b11 << 39);
        let timestamp_part = milliseconds_subtracted & ((1 << 41) - 1);

        let rand_part: u64 = rand::rng().random_range(0..(1<<23));
        let id_u64 = (timestamp_part << 23) | rand_part;

        GameId(bs58::encode(id_u64.to_be_bytes()).into_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<String> for GameId {
    type Error = &'static str;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.len() == 10 || s.len() == 11 {
            Ok(GameId(s))
        }
        else {
            Err("Invalid game id.")
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn try_from_test() {
        let ten_chars = GameId::try_from("abcdefghij".to_string());
        let eleven_chars = GameId::try_from("12345678910".to_string());
        assert!(ten_chars.is_ok());
        assert!(eleven_chars.is_ok());
    }

    #[test]
    fn try_from_fail_test() {
        let too_short = GameId::try_from("123456789".to_string());
        assert!(too_short.is_err());
    }
}