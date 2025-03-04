import { createBrowserRouter } from "react-router-dom";
import { RootLayout } from "./layouts/RootLayout";
import { LocalGame } from "./pages/LocalGame.tsx"
import { OnlineGame } from "./pages/OnlineGame.tsx";
/*import { ErrorPage } from "./pages/ErrorPage";*/
import { About } from "./pages/About";

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
            }
        ]
    }
]);