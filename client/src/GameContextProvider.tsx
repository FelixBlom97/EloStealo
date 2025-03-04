import {createContext, useContext, useState} from "react";
import {Color, GameState} from "./types";

type Props = React.PropsWithChildren;

type ContextType = {
  gameState: GameState | undefined,
  setGameState: (gameState: GameState) => void;
  gameType: string;
  setGameType: (gameType: string) => void;
  roomCode: string;
  setRoomCode: (roomCode: string) => void;
  color: Color;
  setColor: (name: Color) => void;
}

const GameContext = createContext<ContextType | undefined>(undefined);

export const GameContextProvider = (props: Props) => {
  const { children } = props;
  const [gameState, setGameState] = useState<GameState | undefined>(undefined);
  const [gameType, setGameType] = useState("none");
  const [roomCode, setRoomCode] = useState("");
  const [color, setColor] = useState<Color>("white");
  return <GameContext.Provider value={{
    gameState: gameState,
    setGameState: setGameState,
    gameType: gameType,
    setGameType: setGameType,
    roomCode: roomCode,
    setRoomCode: setRoomCode,
    color: color,
    setColor: setColor,
}}>{children}</GameContext.Provider>
}

export const useGameContext = () => {
  const context = useContext(GameContext);
  if (context === undefined) {
      throw new Error('Failed to create context');
  }

  return context;
}