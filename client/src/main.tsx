import React from 'react'
import ReactDOM from 'react-dom/client'
import { GameContextProvider } from './GameContextProvider.tsx'
import './index.css'
import { RouterProvider } from 'react-router-dom'
import { router } from './router.tsx'
import {socket, SocketContext} from "./SocketContext.tsx";

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <GameContextProvider>
        <SocketContext.Provider value={socket}>
            <RouterProvider router={router} />
        </SocketContext.Provider>
    </GameContextProvider>
  </React.StrictMode>,
)


