import classNames from "classnames";
import { ChangeEventHandler } from "react";

type Props = {
    id: string;
    label: string;
    inputType: "text" | "number";
    value: string | number;
    onChange: ChangeEventHandler<HTMLInputElement>;
    hasError: boolean;
};

export const FormInput = (props : Props) => {
        const {id, label, value, inputType, onChange, hasError} = props;

        return ( <div> <div className="relative">
            <label htmlFor={id} className={classNames(
                "text-lg",
            )}>
            {label}
            </label>
            <input type={inputType} id={id} className={classNames("bg-white",
                "px-4 py-2",
                "rounded-lg",
                "w-1/2",
                "h-2/5",
                "text-lg",
                "border-2",
                "focus:outline-none",
                "focus:ring-0",
                { "border-gray-600 focus:border-neutral-300": !hasError },
                { "border-red-600 focus:border-red-500": hasError },
            )} placeholder=" " onChange={onChange} value={value} />
            
            

            </div></div> )
}