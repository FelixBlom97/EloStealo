export type GameState = {
    board: string;
    moves: string[];
    result: "none" | "white" | "black" | "draw"
}

export function isGameState(gameState: unknown): gameState is GameState {
    return (
        typeof gameState === "object" &&
        gameState !== null &&
        "board" in gameState &&
        "moves" in gameState &&
        "result" in gameState
    );
}

export type GameInfoType = {
    game_type: "local" | "online",
    white: string,
    black: string,
    white_elo: number,
    black_elo: number,
    white_stealo: number,
    black_stealo: number,
}

export function isGameInfoType(gameInfo: unknown): gameInfo is GameInfoType {
    return (
        typeof gameInfo === "object" &&
        gameInfo !== null &&
        "game_type" in gameInfo &&
        "white" in gameInfo &&
        "black" in gameInfo &&
        "white_elo" in gameInfo &&
        "black_elo" in gameInfo &&
        "white_stealo" in gameInfo &&
        "black_stealo" in gameInfo
    );
}

export type SocketMessage = GameState | GameInfoType;

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

export type Color = "white" | "black"