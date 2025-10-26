import { createBrowserRouter } from "react-router-dom";
import { RootLayout } from "./components/RootLayout.tsx";
import { LocalGame } from "./pages/LocalGame.tsx"
import { OnlineGame } from "./pages/OnlineGame.tsx";
/*import { ErrorPage } from "./pages/ErrorPage";*/
import { About } from "./pages/About";
import {Game} from "./pages/Game.tsx";

export const router = createBrowserRouter([
    {
        path: "/",
        element: <RootLayout />,
        //errorElement: <ErrorPage />,
        children: [
            {
                index: true,
                element: <LocalGame />
            },
            {
                path: "online",
                element: <OnlineGame />
            },
            {
                path: "about",
                element: <About />
            },
            {
                path: "game/:roomId",
                element: <Game />
            }
        ]
    }
]);