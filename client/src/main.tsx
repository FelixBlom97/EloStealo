//import React from 'react'
import ReactDOM from 'react-dom/client'
import { GameContextProvider } from './GameContextProvider.tsx'
import './index.css'
import { RouterProvider } from 'react-router-dom'
import { router } from './router.tsx'

ReactDOM.createRoot(document.getElementById('root')!).render(
  //<React.StrictMode> annoying when doing sockets
    <GameContextProvider>
        <RouterProvider router={router} />
    </GameContextProvider>
  //</React.StrictMode>,
)


