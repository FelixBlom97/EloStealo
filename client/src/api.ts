import {Color, GameInfoType, GameState, StealoRule} from "./types";

// Local play
export async function startGame(player1: string, player2: string, elo1: number, elo2: number, stealo1: number, stealo2: number) {
    const response = await fetch("/api/startgame", {
        method: "POST",
        headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            player1: player1,
            player2: player2,
            elo1: elo1,
            elo2: elo2,
            stealo1: stealo1,
            stealo2: stealo2
        }),
    });
    if (response.ok) {
        const gamestate = await response.json();
        return gamestate as GameState;
    } else {
        return {
            statusCode: response.status,
            statusText: response.statusText
        }
    }
}

export async function play(move: string, color?: Color) {
    const response = await fetch("/api/play", {
        method: "POST",
        headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
        },
        body:JSON.stringify({
            play_move: move,
            color: color
        })
    });
    if (response.ok) {
        const gamestate = await response.json();
        return gamestate as GameState;
    } else {
        return {
            statusCode: response.status,
            statusText: response.statusText
        }
    }
}

export async function get_stealo_rules() {
    const response = await fetch("/api/rules");
    if (response.ok) {
        const data = await response.json();
        return data as StealoRule[];
    } else {
        const data : StealoRule[] = [];
        return data as StealoRule[];
    }
}

export async function get_local_game_info() {
    const response = await fetch("/api/get_local_info");
    if (response.ok) {
        const data = await response.json();
        return data as GameInfoType;
    } else {
        return {
            statusCode: response.status,
            statusText: response.statusText,
        }
    }
}

// Online play
export async function get_game_info( roomcode: string, color: Color) {
    const response = await fetch("/api/get_game_info", {
        method: "POST",
        headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            roomcode: roomcode,
            color: color,
        }),
    });
    if (response.ok) {
        const gameInfo = await response.json();
        return gameInfo as GameInfoType;
    } else {
        return {
            statusCode: response.status,
            statusText: response.statusText,
        }
    }
}

export async function start_online(roomcode: string, player1: string, player2: string, elo1: number, elo2:number, stealo1: number, stealo2: number) {
    const response = await fetch("/api/start_online", {
        method: "POST",
        headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            roomcode: roomcode,
            player1: player1,
            player2: player2,
            elo1: elo1,
            elo2: elo2,
            stealo1: stealo1,
            stealo2: stealo2
        }),
    });
    if (response.ok) {
        const gamestate = await response.json();
        return gamestate as GameState;
    } else {
        return {
            statusCode: response.status,
            statusText: response.statusText
        }
    }
}
