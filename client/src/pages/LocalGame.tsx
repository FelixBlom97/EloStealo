import { useGameContext } from "../GameContextProvider.tsx";
import { Play } from "./play";
import { Start } from "./start";

export const LocalGame = () => {
    const { gameState, gameType } = useGameContext();
    return (gameState && gameType == "local") ? <Play /> : <Start />;
};
