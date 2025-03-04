import {Piece} from "react-chessboard/dist/chessboard/types";
import {StealoRule} from "./types.ts";


export function random_stealo(elo1: string, elo2: string, stealo1: number, stealo2: number, rules: StealoRule[]): [number, number] {
    if (!Number(elo1) || !Number(elo2)) {return [stealo1, stealo2]}
    const e1 = (Number(elo1) >= Number(elo2)) ? Number(elo1) : Number(elo2);
    const e2 = (Number(elo1) >= Number(elo2)) ? Number(elo2) : Number(elo1);
    const difference = e1 - e2;
    let rules_filtered = rules.filter(rule => rule.elo >= difference);
    if (rules_filtered.length == 0) {
        rules_filtered = rules.filter(rule => rule.elo == 1500);
    }
    const rule1 = rules_filtered[Math.floor(Math.random() * rules_filtered.length)];
    rules_filtered = rules.filter(rule => rule.elo <= rule1.elo - difference + 150 && rule.elo >= rule1.elo - difference - 150);
    if (rules_filtered.length == 0) {
        rules_filtered = rules.filter(rule => rule.elo == 0);
    }
    const rule2 = rules_filtered[Math.floor(Math.random() * rules_filtered.length)];
    return (Number(elo1) == e1) ? [rule1.id, rule2.id]: [rule2.id, rule1.id];
}

export function format_promotion_piece(piece: Piece): string {
    switch (piece) {
        case "wN":
        case "bN": {return "n";}
        case "wB":
        case "bB": {return "b";}
        case "wR":
        case "bR": {return "r";}
        case "wQ":
        case "bQ": {return "q";}
        default: {return "";}
    }
}