import classNames from "classnames";
import { To, useNavigate } from "react-router-dom";

type Props = {
    to: To;
    text: string;
    isActive: boolean;
}

export const NavButton = (props: Props) => {
    const { to, text, isActive } = props;
    const navigate = useNavigate();

    return (<button className={classNames(
        "px-5 hover:bg-gray-200 font-bold h-max py-1 text-lg",
        { "bg-gray-400": !isActive },
        { "bg-gray-300": isActive })}
        onClick={() => navigate(to)}>
        {text}
    </button>)
}