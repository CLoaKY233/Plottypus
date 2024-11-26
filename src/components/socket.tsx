import * as React from "react";
import { useState, useEffect, useRef } from "react";

const SocketDataChart = () => {
  const [text, setText] = useState('');
  const socketRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    // Create socket connection
    const socketUrl = "ws://127.0.0.1:8080";
    const socket = new WebSocket(socketUrl);

    socketRef.current = socket;

    socket.onmessage = async (event: MessageEvent) => {
      try {
        // Parse the incoming data
        const data = JSON.parse(event.data);
        console.log(`Received: ${JSON.stringify(data)}`);
        
        setText(JSON.stringify(data));  // Ensure that 'data' is valid before updating state
        // setSocketData(prevData => {
          //   const newData = [...prevData, {
          //     ...data,
          //     formattedTimestamp: new Date(data.timestamp).toLocaleTimeString()
          //   }].slice(-20);
          //   return newData;
          // });
      
      } catch (error) {
        console.error("Error processing WebSocket message:", error);
      }
    };

    socket.onopen = () => {
      console.log('Connected to WebSocket server');
    };

    socket.onclose = () => {
      console.log("Disconnected from WebSocket server");
    };

    return () => {
      if (socketRef.current) {
        socketRef.current.close();
      }
    };
  }, []);

  return (
    <h1>{text}</h1>
  );
}

export default SocketDataChart;