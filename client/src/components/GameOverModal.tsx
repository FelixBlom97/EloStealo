type GameOverModalProps = {
    result: "white" | "black" | "draw" | "none";
    player1: string;
    player2: string;
}

export const GameOverModal = (props: GameOverModalProps) => {
    const { result, player1, player2 }: GameOverModalProps = props;
    const visible = result !== "none";

    let message: string;
    if (result === "white") {
        message = `${player1} wins!`;
    } else if (result === "black") {
        message = `${player2} wins!`;
    } else {
        message = "It's a draw!";
    }

    return (
        visible &&
        <div className="p-10 absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 text-7xl z-10 font-bold
                        bg-gray-200 rounded-lg bg-opacity-50 text-center">
            {message}
            <button
                    className="text-xl rounded-full bg-slate-700 text-white bg-opacity-100 p-2 w-full hover:bg-slate-500"
                    onClick={() => {
                        window.location.reload()
                    }}>New game
               </button>
            </div>
        )
    ;
}