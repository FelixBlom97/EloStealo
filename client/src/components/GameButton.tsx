
import {Color} from "../types.ts";

type Props = {
    text: string,
    color: Color,
    play_move: (move: string, color: Color) => void,
}

export const GameButton = (props: Props) => {
    const { text, color, play_move } = props;

    const css_button = "w-full text-2xl h-full rounded-lg border-2 grow font-bold border-gray-600 bg-gray-300 hover:bg-white";

    const action = async () => {
        if (text == "Resign") {
            if (confirm("Are you sure?")) {
                play_move("resign", color);
            }
        }
    }

    return (
    <div className="grow flex-1 py-2 px-1">
      <button className={ css_button } onClick={(e) => {e.preventDefault(); action()}}>{ text }</button>
    </div>
    )

}
