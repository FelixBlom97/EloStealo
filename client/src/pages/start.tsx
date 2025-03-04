import { useEffect, useState } from "react";
import {useGameContext} from "../GameContextProvider.tsx";
import { StealoRule, isGameState } from "../types";
import { FormInput } from "../layouts/FormInput";
import {startGame, get_stealo_rules} from "../api";
import { StealoInput } from "../layouts/StealoInput";
import {random_stealo} from "../shared_functions.ts";


export const Start = () => {

    const { setGameState, setGameType } = useGameContext();
    const [player1, setPlayer1] = useState("");
    const [player2, setPlayer2] = useState("");
    const [elo1, setElo1] = useState("");
    const [elo2, setElo2] = useState("");
    const [stealo1, setStealo1] = useState(0);
    const [stealo2, setStealo2] = useState(0);
    const [description1, setDescription1] = useState("Good old normal chess");
    const [description2, setDescription2] = useState("Good old normal chess");
    const [rules, setRules] = useState<StealoRule[]>([]);
    const valid = player1 !== "" && player2 !== "" && player1 !== player2;
    const validElo = (Number(elo1) > 0 || elo1 == "") && (Number(elo2) > 0 || elo2 =="") && ((elo1 !== "" && elo2 !== "") || (elo1 === "" && elo2 === ""));
    
    // Get rules from the database and save them in local storage when rules are not found in local storage
    const set_rules_async = async() => {
        const result = await get_stealo_rules();
        const result_sorted = result.sort((a, b) => a.elo-b.elo);
        localStorage.setItem('rules', JSON.stringify(result));
        setRules(result_sorted);
    }

    function random_rules() {
        if (!(elo1 && elo2)) {
            alert("Please enter both player elos first")
            return
        }
        const [rule1, rule2] = random_stealo(elo1, elo2, stealo1, stealo2, rules);
        setStealo1(rule1);
        setStealo2(rule2);
        setDescription1(get_description(rule1));
        setDescription2(get_description(rule2));
    }

    const start_game = async () => {
        const elo_white = (isNaN(Number(elo1))) ? 0 : Number(elo1);
        const elo_black = (isNaN(Number(elo2))) ? 0 : Number(elo2);
        const result = await startGame(player1, player2, elo_white, elo_black, stealo1, stealo2)
        if (isGameState(result)) {
            setGameState(result);
            setGameType("local");
        }
        else {
            alert("AAAAAAAAAAAAAAH")
        }
    }

    function get_description(id: number): string {
        const get_rule = rules.filter((rule) => {return rule.id===id})[0]
        return get_rule.description;
    }

    useEffect(() => {
        const rules_storage = localStorage.getItem('rules');
        if (rules_storage) {
            const rules_sorted = JSON.parse(rules_storage).sort((a: StealoRule, b: StealoRule) => a.elo-b.elo);
            setRules(rules_sorted);
        }
         else {
             set_rules_async();
        }
    }, []);


    return (
        <div className="">
            <div className="w-1/2 bg-gray-200 mx-auto my-10 border-2 border-gray-300 font-bold text-lg rounded-md">
                <form>
                    <div className="my-4 px-5 inline-flex">
                    <FormInput 
                        id="player1"
                        label="Name of player 1: "
                        inputType="text"
                        value={player1}
                        onChange={e => setPlayer1(e.target.value)}
                        hasError={false}
                    />
                    
                    <FormInput 
                        id="elo1"
                        label="Elo (optional): "
                        inputType="number"
                        value={elo1}
                        onChange={e =>{ setElo1(e.target.value)}}
                        hasError={!validElo}
                    />
                    </div>
                    <div className="my-4 px-5 inline-flex">
                    <FormInput 
                        id="player2"
                        label="Name of player 2: "
                        inputType="text"
                        value={player2}
                        onChange={e => setPlayer2(e.target.value)}
                        hasError={false}
                    />

                    <FormInput 
                        id="elo2"
                        label="Elo (optional): "
                        inputType="number"
                        value={elo2}
                        onChange={e =>{ setElo2(e.target.value)}}
                        hasError={!validElo}
                    />
                    </div >
                    <div className="flex items-end justify-end mr-20">
                        <button className="px-3 rounded-lg text-xl border-gray-600 border-2 bg-gray-300 hover:bg-white"
                        disabled={!validElo} onClick={(event) => { event.preventDefault(); random_rules()}} >Random rules</button>
                    </div>
                    <div>
                        <StealoInput
                            data={rules}
                            player={1}
                            onChange={e =>{ setStealo1(Number(e.target.value)); setDescription1(get_description(Number(e.target.value)));}}
                            title={description1}
                            value ={stealo1}
                            />
                    </div>
                
                    <div>
                        <StealoInput
                            data={rules}
                            player={2} 
                            onChange={e =>{ setStealo2(Number(e.target.value));setDescription2(get_description(Number(e.target.value)))}}
                            title={description2}
                            value={stealo2}
                            />
                    </div>
                    <div className="flex items-center justify-center">
                        <button className="px-5 py-1 mb-5 mt-3 rounded-lg text-xl border-gray-600 border-2 bg-gray-300 hover:bg-white"
                        disabled={!valid} onClick={(event) => { event.preventDefault(); start_game()}}>Start game</button>
                    </div>
                </form>
            </div>
        </div>
    )
}