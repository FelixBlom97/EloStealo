import {io} from "socket.io-client";
import {createContext} from "react";

export const socket = io("127.0.0.1:8080/api/socket", {transports: ['websocket']});
export const SocketContext = createContext(socket);