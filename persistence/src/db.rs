use crate::game_model::{chess_game_to_model, model_to_chess_game, GameModel};
use domain::chessgame::ChessGame;
use futures::TryStreamExt;
use mongodb::bson::{doc, spec::BinarySubtype, Binary};
use mongodb::error::Error;
use mongodb::Database;
use serde::{Deserialize, Serialize};

// Games collection

pub async fn save_game(database: Database, id: String, new_game: ChessGame) -> Result<(), Error> {
    let collection = database.collection("games");
    let new_game_binary = Binary {
        subtype: BinarySubtype::Generic,
        bytes: vec![],
    };
    let document = doc! {
        "_id": &id.to_string(),
        "white": new_game.white,
        "black": new_game.black,
        "game": new_game_binary,
        "elo_white": new_game.elo_white,
        "elo_black": new_game.elo_black,
        "filter_id_white": new_game.rule_id_white,
        "filter_id_black": new_game.rule_id_black,
    };
    match collection.insert_one(document, None).await {
        Ok(_insert_one) => Ok(()),
        Err(e) => {
            log::error!("Failed to insert game into database {:?}", &e);
            Err(e)
        }
    }
}

pub async fn load_game(database: &Database, id: &String) -> Result<ChessGame, Error> {
    let collection = database.collection::<GameModel>("games");
    let result = collection.find_one(doc! {"_id":id}, None).await;
    match result {
        Ok(chessgame) => Ok(model_to_chess_game(chessgame.unwrap())),
        Err(e) => {
            log::error!("Failed to fetch chess game from db {:?}", &e);
            Err(e)
        }
    }
}

pub async fn update_game(
    database: Database,
    id: String,
    chess_game: &ChessGame,
) -> Result<(), Error> {
    // let collection = database.collection::<GameModel>("games");
    // let chess_dto = chess_game_to_model(chess_game);
    // match collection
    //     .update_one(
    //         doc! {"_id": id},
    //         doc! {"$set": ""},
    //         None,
    //     )
    //     .await
    // {
    //     Ok(_update_one) => Ok(()),
    //     Err(e) => {
    //         log::error!("Failed to update games collection: {:?}", e);
    //         Err(e)
    //     }
    // }
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct GameInfoMong {
    pub white: String,
    pub black: String,
    pub white_elo: i32,
    pub black_elo: i32,
    pub white_stealo: i32,
    pub black_stealo: i32,
}

// Performs a couple of checks so that certain game info is only sent after the game has ended.
pub async fn load_game_info(
    database: Database,
    room_code: String,
    color: String,
) -> Result<GameInfoMong, Error> {
    let game_collection = database.collection::<GameModel>("games");
    let chess_dto = game_collection
        .find_one(doc! {"_id":room_code}, None)
        .await?;
    let chessgame = model_to_chess_game(chess_dto.unwrap());
    let game_has_ended = chessgame.get_moves().is_empty();
    let white_elo = if color == "white" || game_has_ended {
        chessgame.elo_white
    } else {
        0
    };
    let white_stealo = if color == "white" || game_has_ended {
        chessgame.rule_id_white
    } else {
        0
    };
    let black_elo = if color == "black" || game_has_ended {
        chessgame.elo_black
    } else {
        0
    };
    let black_stealo = if color == "black" || game_has_ended {
        chessgame.rule_id_black
    } else {
        0
    };
    Ok(GameInfoMong {
        white: chessgame.white,
        black: chessgame.black,
        white_elo,
        black_elo,
        white_stealo,
        black_stealo,
    })
}

// Elo stealo rules Collection

#[derive(Serialize, Deserialize)]
pub struct StealoRuleMong {
    pub id: i32,
    pub name: String,
    pub elo: i32,
    pub description: String,
}

pub async fn get_stealo_rules(database: Database) -> Vec<StealoRuleMong> {
    let collection = database.collection::<StealoRuleMong>("rules");
    let cursor = match collection.find(None, None).await {
        Ok(cursor) => cursor,
        Err(e) => {
            log::error!("Failed to fetch stealo rules: {:?}", e);
            return vec![];
        }
    };
    cursor.try_collect().await.unwrap_or_else(|_| vec![])
}
