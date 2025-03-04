import {Color, StealoRule} from "../types.ts";
import {GameButton} from "./GameButton.tsx";

type Props = {
    player1: string,
    player2: string,
    elo1: number,
    elo2: number,
    stealo1: number,
    stealo2: number,
    result: string,
    play_move: (move: string, color: Color) => void,
}

export const GameInfo = (props: Props) => {
    const { player1, player2, elo1, elo2, stealo1, stealo2, result, play_move } = props;
    const rules = localStorage.getItem("rules");
    const rule1 = rules? JSON.parse(rules).filter((rule: StealoRule) => {return rule.id===stealo1})[0]
        : {name: "Couldn't get rule", description: "", elo: ""};
    const rule2 = rules? JSON.parse(rules).filter((rule: StealoRule) => {return rule.id===stealo2})[0]
        : {name: "Couldn't get rule", description: "", elo: ""};
    const stealo_css = (result != "none") ? "basis-2/12 my-2 text-xl break-words"
        : "bg-gray-600 text-gray-600 hover:text-black hover:bg-gray-200 basis-2/12 my-2 text-xl break-words";
    const reveal_instruction = (result == "none") ? "Hover to reveal stealo" : ""

    return (
    <div className="h-full w-full px-3 py-2 bg-gray-200 border-2 border-gray-600 rounded-lg flex flex-col">
        <div className="basis-1/12 my-2 text-2xl font-bold break-words" > {player2}  ({elo2})</div>
        <div className={stealo_css}>{rule2.name} ({rule2.elo}): <br />
            {rule2.description}</div>
        <div className="basis-2/12 my-2 flex flex-row border-2" >
            <GameButton text={"Offer draw"} color={"black"} play_move={play_move}/>
            <GameButton text={"Resign"} color={"black"} play_move={play_move}/></div>
        <div className="basis-1/12 my-2 flex justify-center text-sm pt-3" >{reveal_instruction} </div>
        <div className="basis-2/12 my-2 flex flex-row">
            <GameButton text={"Offer draw"} color={"white"} play_move={play_move}/>
            <GameButton text={"Resign"} color={"white"} play_move={play_move}/></div>
        <div className="basis-1/12 my-2 text-2xl font-bold break-words" > {player1}  ({elo1})</div>
        <div className={stealo_css} > {rule1.name} ({rule1.elo}): <br />
            {rule1.description}</div>
    </div>
    )
}

export const GameInfoOnline = (props: Props) => {
    const { player1, player2, elo1, elo2, stealo1, stealo2, result } = props;
    const elo_p1 = (elo1 != 0)? elo1 : "???";
    const elo_p2 = (elo2 != 0 && result != "none")? elo2 : "???";
    const rules = localStorage.getItem("rules");
    const rule1 = rules? JSON.parse(rules).filter((rule: StealoRule) => {return rule.id===stealo1})[0]
        : {name: "???", description: "Rules are revealed when the game ends", elo: "???"};
    const rule2 = (rules && result != "none")? JSON.parse(rules).filter((rule: StealoRule) => {return rule.id===stealo2})[0]
        : {name: "???", description: "Rules are revealed when the game ends", elo: "???"};

    return (
        <div className="h-full w-full px-3 py-2 bg-gray-200 border-2 border-gray-600 rounded-lg flex flex-col">
            <div className="basis-1/12 my-2 text-2xl font-bold break-words" > {player2}  ({elo_p2})</div>
            <div className="basis-2/12 my-2 text-xl break-words">{rule2.name} ({rule2.elo}): <br />
                {rule2.description}</div>
            <div className="basis-2/12 my-2" ></div>
            <div className="basis-1/12 my-2 flex justify-center text-sm pt-3" ></div>
            <div className="basis-2/12 my-2" ></div>
            <div className="basis-1/12 my-2 text-2xl font-bold break-words" > {player1}  ({elo_p1})</div>
            <div className="basis-2/12 my-2 text-xl break-words" > {rule1.name} ({rule1.elo}): <br />
                {rule1.description}</div>
        </div>
    )
}