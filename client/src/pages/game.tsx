import {useEffect, useRef, useState} from "react";
import {useParams} from "react-router-dom";
import {GameInfoType, GameState, isGameInfoType, isGameState, SocketMessage} from "../types.ts";
import {Chessboard} from "react-chessboard";
import {Piece, Square} from "react-chessboard/dist/chessboard/types";
import {format_promotion_piece} from "../shared_functions.ts";
import {GameInfo} from "../components/GameInfo.tsx";
import { GameOverModal } from "../components/GameOverModal.tsx";

export const Game = () => {
    const { roomId } = useParams();
    const initialGameState: GameState = { board: "", moves: [], result: "none"}
    const [gameState, setGameState] = useState<GameState>(initialGameState)
    const board: string = gameState.board;
    const result: "none" | "white" | "black" | "draw" = gameState.result;
    const moves: string[] = gameState.moves;
    let drag_pawn: boolean = false;
    const initialInfo: GameInfoType = {  game_type:"local", white:"", black: "", white_elo: 0, black_elo: 0, white_stealo: 0, black_stealo: 0 }
    const [gameInfo, setGameInfo] = useState<GameInfoType>(initialInfo)

    const websocket = useRef<WebSocket | null>(null);
    useEffect(() => {
        if(!roomId) return;

        const wsUrl = `ws://localhost:8080/ws/${roomId}`;
        websocket.current = new WebSocket(wsUrl);

        websocket.current.onopen = () => {
            console.log(`Connected to WebSocket room: ${roomId}`);
        };

        websocket.current.onmessage = (event) => {
            const data = JSON.parse(event.data) as SocketMessage;
            console.log('Received data:', data);
            if (isGameState(data)) {
                setGameState(data)
            }
            if (isGameInfoType(data)) {
                setGameInfo(data)
            }
        };

        websocket.current.onerror = (error) => {
            console.error('WebSocket error:', error);
        };

        websocket.current.onclose = (event) => {
            console.log('WebSocket connection closed:', event.reason, `Code: ${event.code}`);
        };

    }, [roomId])

    function is_piece_a_pawn(piece: Piece) {
        drag_pawn = piece == 'wP' || piece == 'bP';
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

    function move(move: string): boolean {
        if (websocket.current && websocket.current.readyState === WebSocket.OPEN && moves.includes(move)) {
            websocket.current.send(move)
            return true;
        }
        return false;
    }

    return (
        <div className="w-3/5 mx-auto mt-10 flex flex-row">
            <GameOverModal result={result} player1={gameInfo.white} player2={gameInfo.black} />
            <div className="w-2/3 flex flex-col items-center justify-center">
                <Chessboard 
                    position={board} 
                    onPieceDragBegin={(piece) => is_piece_a_pawn(piece)}
                    onPieceDrop={(sourceSquare, targetSquare, piece) => move(format_move(sourceSquare, targetSquare, piece))}
                    animationDuration={80}
                    arePiecesDraggable={result === "none"}
                />
            </div>
            <div className="w-1/3">
                <GameInfo 
                    game_type={gameInfo.game_type}
                    player1={gameInfo.white}
                    player2={gameInfo.black}
                    elo1={gameInfo.white_elo} 
                    elo2={gameInfo.black_elo}
                    stealo1={gameInfo.white_stealo}
                    stealo2={gameInfo.black_stealo}
                    result={result}
                />
            </div>
        </div>
    )
}