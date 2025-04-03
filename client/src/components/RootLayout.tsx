import { Outlet, useLocation } from "react-router-dom";
import { NavButton } from "./NavButton";

export const RootLayout = () => {
    const { pathname } = useLocation();

    return <div>
        <header className="p-3 bg-gray-500 text-white text-7xl text-center font-bold font-sans">Elo Stealo
        <p className="text-lg">Stupid rules for fairer chess</p></header>
        <nav className="flex items-center justify-center bg-gray-400 ">
            <NavButton to="/" text="Local game" isActive={pathname === "/"} />
            <NavButton to="/online" text="Play a friend" isActive={pathname === "/online"} />   
            <NavButton to="/about" text="About" isActive={pathname === "/about"} />
        </nav>
        <main className="flex-1">
            <Outlet />
        </main>

    </div>
};