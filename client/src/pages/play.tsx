import { Chessboard } from "react-chessboard"
import { useGameContext} from "../GameContextProvider.tsx"
import {Square, Piece} from "react-chessboard/dist/chessboard/types";
import {get_local_game_info, play} from "../api"
import {isGameState, isGameInfoType, GameInfoType, Color} from "../types.ts";
import {format_promotion_piece} from "../shared_functions.ts";
import {useEffect, useState} from "react";
import {GameInfo} from "../layouts/GameInfo.tsx";

export const Play = () => {
    const {gameState, setGameState} = useGameContext();
    const board = gameState?.board;
    const moves = gameState?.moves;
    const result = gameState?.result;
    const initialInfo: GameInfoType = {  white:"", black: "", white_elo: 0, black_elo: 0, white_stealo: 0, black_stealo: 0 }
    const [gameInfo, setGameInfo] = useState<GameInfoType>(initialInfo)
    let text = ""
    let drag_pawn: boolean = false; // Used to check if a pawn is being promoted this move.

    function is_piece_a_pawn(piece: Piece) {
        if (piece == 'wP' || piece == 'bP') {
            drag_pawn = true
        } else {
            drag_pawn = false
        }
    }

    function format_move(source: Square, target: Square, piece: Piece): string {
        let move = source + target;
        if ((source[1] == '7' && target[1] == '8') || (source[1] == '2' && target[1] == '1')) {
            if (drag_pawn) {
                move += format_promotion_piece(piece);
            }
        }
        return move
    }

    async function move(move: string, color?: Color) {
        if (!moves) {
            return false;
        } else if (moves.includes(move) || move == "resign" ) {
            const new_position = await play(move, color)
            console.log(new_position);
            if (isGameState(new_position)) {
                setGameState(new_position);
                return true;
            }
            return false;
        } else {
            return false;
        }
    }

    const set_game_info = async () => {
        const gameInfo = await get_local_game_info();
        if (isGameInfoType(gameInfo)) {
            setGameInfo(gameInfo);
        } else {
            alert("What did you dooooooooo?");
        }
    }

    useEffect(() => {
        set_game_info()
    },[])


    //<div className="text-2xl text-gray-700 font-semibold">{ player2 }</div>
    //<div className="text-2xl text-gray-700 font-semibold">{player1}</div>
    if (result == "none") {
        return (
        <div className="w-3/5 mx-auto mt-10 flex flex-row">
            <div className="w-2/3 flex flex-col items-center justify-center">
                <Chessboard position={board} onPieceDragBegin={(piece) => is_piece_a_pawn(piece)}
                            onPieceDrop={(sourceSquare, targetSquare, piece) => {
                                move(format_move(sourceSquare, targetSquare, piece));
                                return false
                            }}
                            animationDuration={80}/>
            </div>
            <div className="w-1/3">
                <GameInfo player1={gameInfo.white} player2={gameInfo.black}
                          elo1={gameInfo.white_elo} elo2={gameInfo.black_elo}
                          stealo1={gameInfo.white_stealo} stealo2={gameInfo.black_stealo} result={result} play_move={move}/>
            </div>
        </div>)
    } else if (result != undefined) {
        if (result == "white") {
            text = gameInfo.white + " wins!"
        } else if (result == "black") {
            text = gameInfo.black + " wins!"
        } else {
            text = "It's a draw!"
        }
        return (<div>
            <div className="p-10 absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 text-7xl z-10 font-bold
        bg-gray-200 rounded-lg bg-opacity-50 text-center">
                {text}
                <button
                    className="text-xl rounded-full bg-slate-700 text-white bg-opacity-100 p-2 w-full hover:bg-slate-500"
                    onClick={() => {
                        window.location.reload()
                    }}>New game
                </button>
            </div>
            <div className="w-3/5 mx-auto mt-10 flex flex-row">
                <div className="w-2/3 flex flex-col items-center justify-center">
                    <Chessboard position={board} arePiecesDraggable={false}/>
                </div>
                <div className="w-1/3">
                    <GameInfo player1={gameInfo.white} player2={gameInfo.black}
                              elo1={gameInfo.white_elo} elo2={gameInfo.black_elo}
                              stealo1={gameInfo.white_stealo} stealo2={gameInfo.black_stealo} result={result} play_move={move}/>
                </div>
            </div>
        </div>)
    }
}