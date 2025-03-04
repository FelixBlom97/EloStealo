export type GameState = {
    board: string;
    moves: string[];
    result: "none" | "white" | "black" | "draw"
}

export type Color = "white" | "black"

export function isGameState(gameState: unknown): gameState is GameState {
    return (gameState as GameState) !== undefined;
}

export type StealoRule = {
    id: number;
    name: string;
    elo: number;
    description: string;
}

export type OnlineMove = {
    roomcode: string,
    play_move: string
}

export type GameInfoType = {
    white: string,
    black: string,
    white_elo: number,
    black_elo: number,
    white_stealo: number,
    black_stealo: number,
}

export function isGameInfoType(gameInfo: unknown): gameInfo is GameInfoType {
    return (gameInfo as GameInfoType) !== undefined;
}