import { useState, useEffect } from 'react';

function useGameWebSocket(gameId: string) {
    const [messages, setMessages] = useState<string[]>([]);
    const [ws, setWs] = useState<WebSocket | null>(null);

    useEffect(() => {
        const socket = new WebSocket(`ws://127.0.0.1:8080/api/ws/new_sockets/${gameId}`);

        socket.onmessage = (event) => {
            setMessages((prev) => [...prev, event.data]);
        };

        socket.onclose = () => console.log("WebSocket closed");

        setWs(socket);

        return () => socket.close();
    }, [gameId]);

    const sendMessage = (message: string) => {
        if (ws && ws.readyState === WebSocket.OPEN) {
            ws.send(message);
        }
    };

    return { messages, sendMessage };
}

export default useGameWebSocket;