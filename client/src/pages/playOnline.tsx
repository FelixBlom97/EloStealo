import {useState, useEffect, useContext} from "react";
import {Chessboard} from "react-chessboard";
import {useGameContext} from "../GameContextProvider.tsx";
import {get_game_info} from "../api.ts";
import {isGameState, GameInfoType, OnlineMove, isGameInfoType, Color} from "../types.ts";
import {Piece, Square} from "react-chessboard/dist/chessboard/types";
import {format_promotion_piece} from "../shared_functions.ts";
import {SocketContext} from "../SocketContext.tsx";
import {GameInfoOnline} from "../layouts/GameInfo.tsx";

export const Play = () => {
    const websocket = useContext(SocketContext);
    const {gameState, setGameState, roomCode, color} = useGameContext();
    const board = gameState?.board;
    const initialInfo: GameInfoType = {  white:"", black: "", white_elo: 0, black_elo: 0, white_stealo: 0, black_stealo: 0 }
    const [gameInfo, setGameInfo] = useState<GameInfoType>(initialInfo)
    const player1 = (color === "white") ? gameInfo.white : gameInfo.black;
    const player2 = (color === "white") ? gameInfo.black : gameInfo.white;
    const elo1 = (color === "white") ? gameInfo.white_elo : gameInfo.black_elo;
    const elo2 = (color === "white") ? gameInfo.black_elo : gameInfo.white_elo;
    const stealo1 = (color === "white") ? gameInfo.white_stealo : gameInfo.black_stealo;
    const stealo2 = (color === "white") ? gameInfo.black_stealo : gameInfo.white_stealo;
    const moves = gameState ? gameState.moves : [];
    const result = gameState ? gameState.result : "none";
    const draggable = (color === "white" && board?.split(" ")[1] === 'w') || (color === "black" && board?.split(" ")[1] === 'b');
    let drag_pawn: boolean = false; // Used to check if a pawn is being promoted this move.
    let resultText = "";

    const set_game_info = async() => {
        const game_info = await get_game_info(roomCode, color);
        if (isGameInfoType(game_info)) {
            setGameInfo(game_info);
        } else {
            alert("Everything is on fire.")
        }
    }

    async function play_move(move: string) {
        if (!moves) {
            return false;
        }
        else if (moves.includes(move)) {
            const online_move: OnlineMove = { roomcode: roomCode, play_move: move}
            websocket.emit("move", online_move)
            return false;
        }
        else {
            return false;
        }
    }

    function format_move(source: Square, target: Square, piece: Piece) {
        let move = source + target;
        if ((source[1]=='7' && target[1]=='8') || (source[1]=='2' && target[1]=='1')) {
            if (drag_pawn) {
                move += format_promotion_piece(piece);
            }
        }
        return move;
    }

    function is_piece_a_pawn(piece: Piece) {
        drag_pawn = piece == 'wP' || piece == 'bP';
    }

    function filler_play_move(_move: string, _color: Color) {}

    // Dependency on result to request game info again once the game is finished.
    // Server send full game info on a finished game and partial info on an ongoing one.
    useEffect(() => {
        set_game_info()
    }, [result])

    useEffect( () =>{
        websocket.on("connected", () => {
            if ( roomCode != "" ) { websocket.emit("reconnected", roomCode) }
        });
        websocket.on("disconnected", () => { websocket.emit("disconnect_timer", roomCode) });
    }, [websocket, roomCode])

    useEffect( () => {
        websocket.on("sync", (arg) => {
            if (isGameState(arg)) {
                setGameState(arg);
            }
        });
        websocket.on("abandon", () => {
            if (result == "none") {alert("Opponent abandoned the game"); location.reload()}
        });
    }, [websocket])

    if (result == "none") {
        return (
            <div className="w-3/5 mx-auto mt-10 flex flex-row">
                <div className="w-2/3 flex flex-col items-center justify-center">
                    <Chessboard position={board} boardOrientation={color} arePiecesDraggable={draggable}
                                animationDuration={80} onPieceDragBegin={(piece) => is_piece_a_pawn(piece)}
                                onPieceDrop={(sourceSquare, targetSquare, piece) => {
                                    const move = format_move(sourceSquare, targetSquare, piece);
                                    play_move(move);
                                    return moves.includes(move)
                                }}
                    />
                </div>
                <div className="w-1/3">
                    <GameInfoOnline player1={player1} player2={player2}
                              elo1={elo1} elo2={elo2}
                              stealo1={stealo1} stealo2={stealo2} result={result} play_move={filler_play_move}
                    />
                </div>
            </div>
        )
    } else {
        if (result == "white") {
            resultText = gameInfo.white + " wins!"
        }
        if (result == "black") {
            resultText = gameInfo.black + " wins!"
        }
        if (result == "draw") {
            resultText = "It's a draw!"
        }
        return (<div>
                <div className="p-10 absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 text-7xl z-10 font-bold
             bg-gray-200 rounded-lg bg-opacity-50 text-center"> { resultText }
                <button className="text-xl rounded-full bg-slate-700 text-white bg-opacity-100 p-2 w-full hover:bg-slate-500"
                        onClick={() => {window.location.reload()}}>New game</button>
            </div>
                <div className="w-3/5 mx-auto mt-10 flex flex-row">
                    <div className="w-2/3 flex flex-col items-center justify-center">
                        <Chessboard position={board} boardOrientation={color} arePiecesDraggable={false}/>
                    </div>
                    <div className="w-1/3">
                        <GameInfoOnline player1={player1} player2={player2}
                                  elo1={elo1} elo2={elo2}
                                  stealo1={stealo1} stealo2={stealo2} result={result} play_move={filler_play_move}/>
                    </div>
                </div>
            </div>
        )
    }
}