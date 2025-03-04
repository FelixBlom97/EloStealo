import {useGameContext} from "../GameContextProvider.tsx";
import {FormInput} from "../layouts/FormInput.tsx";
import {StealoInput} from "../layouts/StealoInput.tsx";
import {useContext, useEffect, useState} from "react";
import {GameState, isGameState, StealoRule} from "../types.ts";
import {get_stealo_rules, start_online} from "../api.ts";
import {nanoid} from "nanoid";
import {random_stealo} from "../shared_functions.ts";
import {SocketContext} from "../SocketContext.tsx";

export const Start = () => {
    const websocket = useContext(SocketContext);
    const { setGameState, roomCode, setRoomCode, setGameType, setColor } = useGameContext();
    const [waitForPlayerTwo, setWaitForPlayerTwo] = useState(false);
    const [rules, setRules] = useState<StealoRule[]>([]);
    const [player, setPlayer] = useState("");
    const [elo, setElo] = useState("");
    const [stealo, setStealo] = useState(0);
    const [description1, setDescription1] = useState("Good old normal chess");
    const [code, setCode] = useState("");
    const valid = ( player != "")
    const validElo = (Number(elo) || elo == "");

    const get_rules = async() => {
        const result = await get_stealo_rules()
        const result_sorted = result.sort((a, b) => a.elo-b.elo)
        setRules(result_sorted)
    }

    function get_description(id: number): string {
        const get_rule = rules.filter((rule) => {return rule.id===id})[0]
        return get_rule.description;
    }

    function create_room() {
        setColor("white");
        const newCode = nanoid(8);
        setRoomCode(newCode);
        websocket.emit("create_room", {room: newCode, name: player, elo: elo, stealo: stealo});
    }

    function leave_room() {
        websocket.emit("leave", roomCode)
    }

    function join_room() {
        setColor("black")
        setRoomCode(code);
        websocket.emit("join", {room: code, name: player, elo: elo, stealo: stealo});
    }

    function start_game(result: GameState) {
        websocket.emit("start_game", result)
    }

    useEffect(() => {
        get_rules();
    }, []);

    useEffect(()=>{
        // Creator of the room receives the other player's data and creates the game.
        websocket.on("create_game", async(arg) => {
            setGameType("online");
            if (waitForPlayerTwo) {
                const [stealo1, stealo2]: [number, number] = random_stealo(elo, arg.elo, stealo, arg.stealo, rules);
                const elo1 = (isNaN(Number(elo))) ? 0 : Number(elo);
                const elo2 = (isNaN(Number(arg.elo))) ? 0 : Number(arg.elo);
                const result = await start_online(roomCode, player, arg.name, elo1, elo2, stealo1, stealo2);
                if (isGameState(result)) {
                    setGameState(result);
                    start_game(result);
                } else {
                    alert("Something went wrong ")
                }
            }
        })
    },[websocket, player, roomCode, waitForPlayerTwo]);

    useEffect( () => {
        websocket.on("join",() => { setWaitForPlayerTwo(true)});
        websocket.on("room_not_found", () => { alert("Game not found. You can create a new game with the 'Start game' button")})
        websocket.on("full", () => alert("Game already has 2 players"));
        websocket.on("leave", () => {setWaitForPlayerTwo(false)});
        websocket.on("start_game", (arg) => {
            if (isGameState(arg)) {
                setGameState(arg);
            }
        })
    }, [websocket]);

    // Form to create or join a room
    if (!waitForPlayerTwo) {
        return (<div className="">
            <div className="w-1/2 bg-gray-200 mx-auto my-10 border-2 border-gray-300 font-bold text-lg rounded-md">
                <form>
                    <div className="my-4 px-5 flex items-center justify-center">
                        <FormInput
                            id="player"
                            label="Name: "
                            inputType="text"
                            value={player}
                            onChange={e => setPlayer(e.target.value)}
                            hasError={false}
                        />

                        <FormInput
                            id="elo"
                            label="Elo: "
                            inputType="number"
                            value={elo}
                            onChange={e => setElo(e.target.value)}
                            hasError={!validElo}
                        />
                    </div>

                    <div>
                        <StealoInput
                            data={rules}
                            player={0}
                            onChange={e =>{ setStealo(Number(e.target.value)); setDescription1(get_description(Number(e.target.value)));}}
                            title={description1}
                            value ={stealo}
                        />
                    </div>

                    <div className="my-4 px-5 flex items-center justify-center">
                        <FormInput
                            id="code"
                            label="Join game code: "
                            inputType="text"
                            value={code}
                            onChange={e => setCode(e.target.value)}
                            hasError={false}
                        />
                    </div>
                    <div className="flex items-center justify-center">
                        <button className="px-5 py-1 mb-5 mt-3 mx-5 rounded-lg text-xl border-gray-600 border-2 bg-gray-300 hover:bg-white"
                                disabled={!valid} onClick={(event) => { event.preventDefault();  create_room()}}>Start game</button>
                        <button className="px-5 py-1 mb-5 mt-3 mx-5 rounded-lg text-xl border-gray-600 border-2 bg-gray-300 hover:bg-white"
                                disabled={!valid} onClick={(event) => { event.preventDefault(); join_room()}}>Join game</button>
                    </div>
                </form>
            </div>
        </div>)
    }
    // Waiting for player 2:
    else {
        return ( <div>
            <div className="w-1/2 bg-gray-200 mx-auto my-10 border-2 border-gray-300 font-bold rounded-md flex-col items-center justify-center text-5xl">
                <div className="my-8 items-center text-center">Waiting for opponent to join... </div>
                <div className="mt-8 mb-2 mx-auto items-center text-center">Code: { roomCode }</div>
                <div className="flex items-center justify-center">
                    <button className="px-5 py-1 mb-5 mt-3 mx-5 rounded-lg text-xl border-gray-600 border-2 bg-gray-300 hover:bg-white"
                            onClick={ (event) => {event.preventDefault(); leave_room()}}>Leave</button>
                </div>
            </div>
        </div>)
    }
}