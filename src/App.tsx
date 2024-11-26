import "./App.css"
import { AppSidebar } from "@/components/sidebar"
import { ThemeProvider } from "@/components/theme-provider"
import { GraphComponent } from "@/components/graph-component"
import { SidebarProvider } from "@/components/ui/sidebar"
import { useState, useEffect, useRef } from "react"


export default function Home() {
  const [socketData, setSocketData] = useState<Object[]>();
  const socketRef = useRef<WebSocket | null>(null);
  const [isConnected, setIsConnected] = useState(false);

  // useEffect(() => {
  //   // Create socket connection
  //   const socketUrl = "ws://127.0.0.1:8080";
  //   const socket = new WebSocket(socketUrl);

  //   socketRef.current = socket;

  //   socket.onmessage = async (event: MessageEvent) => {
  //     try {
  //       // Parse the incoming data
  //       const data = JSON.parse(event.data);
  //       console.log(`Received: ${JSON.stringify(data)}`);

  //       setSocketData(prevData => {
  //         const newData = [
  //           ...(prevData || []),
  //           {
  //             ...data,
  //             formattedTimestamp: new Date(data.timestamp).toLocaleTimeString(),
  //           }
  //         ].slice(-100);
  //         return newData;
  //       });

  //     } catch (error) {
  //       console.error("Error processing WebSocket message:", error);
  //     }
  //   };

  //   socket.onopen = () => {
  //     console.log('Connected to WebSocket server');
  //     setIsConnected(true);
  //   };

  //   socket.onclose = () => {
  //     console.log("Disconnected from WebSocket server");
  //     setIsConnected(false);
  //   };

  //   return () => {
  //     if (socketRef.current) {
  //       socketRef.current.close();
  //       setIsConnected(false);
  //     }
  //   };
  // }, []);

  useEffect(() => {
    // Cleanup WebSocket when the component is unmounted
    return () => {
      if (socketRef.current) {
        socketRef.current.close();
      }
    };
  }, []);

  function handleConnectClick() {
    if (socketRef.current && isConnected) {

      socketRef.current.close();
      setIsConnected(false);
    } else if (!socketRef.current || !isConnected) {
      
      const socketUrl = "ws://127.0.0.1:8080";
      const socket = new WebSocket(socketUrl);
      socketRef.current = socket;

      setIsConnected(true);

      socket.onmessage = async (event: MessageEvent) => {
        try {
          const data = JSON.parse(event.data);
          setSocketData(prevData => {
            const newData = [
              ...(prevData || []),
              {
                ...data,
                formattedTimestamp: new Date(data.timestamp).toLocaleTimeString(),
              },
            ].slice(-100);
            return newData;
          });
        } catch (error) {
          console.error("Error processing WebSocket message:", error);
        }
      };

      socket.onopen = () => {
        console.log('Connected to WebSocket server');
        setIsConnected(true);
      };

      socket.onclose = () => {
        console.log("Disconnected from WebSocket server");
        setIsConnected(false);
      };
    }
  }   

  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <SidebarProvider>
        <div className="flex h-screen bg-background text-foreground">
          <AppSidebar isConnected={isConnected} handleConnectClick={handleConnectClick} />
          <div className="flex flex-wrap items-start justify-center gap-2 p-4">
            <div className="flex gap-2 w-full">
              <div className="flex-1 p-2">
                <GraphComponent title="EMG Channel 1" blue={true} data={socketData} />
              </div>
              <div className="flex-1 p-2">
                <GraphComponent title="EMG Channel 2" green={true} data={socketData} />
              </div>
            </div>
            <div className="flex-1 p-4">
              <GraphComponent title="Real Time EMG Visualization" blue={true} green={true} data={socketData} />
            </div>
          </div>
        </div>
      </SidebarProvider>
    </ThemeProvider>
  )
}