import { StealoRule } from "../types";
import { ChangeEventHandler } from "react";

type Props = {
    data: StealoRule[];
    player: number;
    onChange: ChangeEventHandler<HTMLSelectElement>;
    title: string;
    value: number;
}

export const StealoInput = (props: Props) => {
    const { data, player, onChange, title, value} = props;
    const label = (player == 0) ? "Stealo rule" : "Stealo rule for player " + player;

    if (data.length==0) {
        return ( <div></div>)
    } else {
        return ( <div><div className="relative px-5 py-4 max-h-40">
            <span>{label}: </span>
            <select className="w-4/5 bg-white rounded-lg px-4 py-2 border-2 border-gray-600 overflow-scroll" onChange={onChange} 
            title={title} value={value} >
            {data.map(({ id, name, elo, description }) => <option key={id} title={description} value={id} label={name + " (" + elo + ")"} ></option>)}
            </select>

        </div></div>)
    }
} 