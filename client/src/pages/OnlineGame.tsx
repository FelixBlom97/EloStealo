import { Start } from "./startOnline.tsx";
import { Play } from "./playOnline.tsx";
import {useGameContext} from "../GameContextProvider.tsx";

export const OnlineGame = () => {
    const { gameState, gameType } = useGameContext();
    return (gameState && gameType == "online") ? <Play /> : <Start />
};
