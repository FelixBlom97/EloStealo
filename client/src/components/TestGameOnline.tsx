import {useCallback, useEffect, useRef, useState} from 'react';
import { useParams } from "react-router";

type Message = { type: string, text: string }
function GameTest() {
    const gameId = useParams().gameId!;
    const [messages, setMessages] = useState<Message[]>([]); // Array to store chat messages
    const [inputMessage, setInputMessage] = useState<string>(''); // Current message in input field
    const [isConnected, setIsConnected] = useState(false); // Track connection status
    const webSocket = useRef<WebSocket | null>(null); // Ref to hold the WebSocket instance

    // --- WebSocket Connection Logic ---
    const connectWebSocket = useCallback(() => {
        if (!gameId) {
            console.log("Game ID is not set, cannot connect.");
            return;
        }
        if (webSocket.current && webSocket.current.readyState === WebSocket.OPEN) {
            console.log("WebSocket already connected.");
            return;
        }

        // Ensure we are using the correct address and port of your Axum server
        // Use ws:// for non-secure, wss:// for secure connections
        const wsUrl = `ws://127.0.0.1:8080/api/ws/new_sockets/${gameId}`;
        console.log(`Attempting to connect to: ${wsUrl}`);
        webSocket.current = new WebSocket(wsUrl);

        webSocket.current.onopen = () => {
            console.log('WebSocket connection opened');
            setIsConnected(true);
            setMessages(prev => [...prev, { type: 'system', text: 'Connected!' }]);
        };

        webSocket.current.onmessage = (event) => {
            console.log('Message received:', event.data);
            // Assuming server sends plain text messages back
            // In a real app, you'd likely parse JSON: const messageData = JSON.parse(event.data);
            setMessages(prev => [...prev, { type: 'received', text: event.data }]);
        };

        webSocket.current.onerror = (error) => {
            console.error('WebSocket error:', error);
            setMessages(prev => [...prev, { type: 'system', text: 'Connection error!' }]);
        };

        webSocket.current.onclose = (event) => {
            console.log('WebSocket connection closed:', event.code, event.reason);
            setIsConnected(false);
            webSocket.current = null; // Clear the ref
            setMessages(prev => [...prev, { type: 'system', text: 'Disconnected.' }]);
            // Optional: Attempt to reconnect after a delay
            // setTimeout(connectWebSocket, 5000); // Reconnect after 5 seconds
        };
    }, [gameId]); // Re-run connect logic if gameId changes

    // --- Effect to manage connection ---
    useEffect(() => {
        connectWebSocket(); // Connect when component mounts or gameId changes

        // Cleanup function: Close WebSocket connection when component unmounts
        return () => {
            if (webSocket.current) {
                console.log("Closing WebSocket connection on component unmount.");
                webSocket.current.close();
            }
        };
    }, [connectWebSocket]); // Dependency array includes the memoized connect function

    // --- Send Message Handler ---
    const sendMessage = () => {
        if (webSocket.current && webSocket.current.readyState === WebSocket.OPEN && inputMessage.trim()) {
            console.log('Sending message:', inputMessage);
            // Send the message as a string. If your backend expects JSON, stringify an object:
            // webSocket.current.send(JSON.stringify({ text: inputMessage }));
            webSocket.current.send(inputMessage);
            // Optionally add the sent message directly to the list
            // setMessages(prev => [...prev, { type: 'sent', text: inputMessage }]);
            setInputMessage(''); // Clear input field
        } else {
            console.log('WebSocket not connected or message is empty.');
        }
    };

    // --- Render ---
    return (
        <div className="App">
            <h1>Real-Time Chat (Game ID: {gameId})</h1>
            <p>Status: {isConnected ? 'Connected' : 'Disconnected'}</p>

            {/* Area to display messages */}
            <div className="message-area">
                {messages.map((msg, index) => (
                    <p key={index} className={`message ${msg.type}`}>
                        {msg.text}
                    </p>
                ))}
            </div>

            {/* Input area */}
            <div className="input-area">
                <input
                    type="text"
                    value={inputMessage}
                    onChange={(e) => setInputMessage(e.target.value)}
                    onKeyPress={(e) => e.key === 'Enter' && sendMessage()} // Send on Enter key
                    disabled={!isConnected} // Disable input if not connected
                />
                <button onClick={sendMessage} disabled={!isConnected}>
                    Send
                </button>
            </div>

            {/* Example: Allow changing game ID (would trigger reconnect) */}
            {/*
      <div>
        <input type="text" value={gameId} onChange={(e) => setGameId(e.target.value)} placeholder="Enter Game ID"/>
      </div>
      */}
        </div>
    );
}

export default GameTest;