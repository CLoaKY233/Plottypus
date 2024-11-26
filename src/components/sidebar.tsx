import {
  RotateCcw,
  ChevronsLeft,
  ChevronsRight,
  Unplug,
  Wifi,
} from 'lucide-react'
import {
  Sidebar,
  SidebarContent,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuItem,
} from "@/components/ui/sidebar"
import { Button } from './ui/button'
import Logo from "@/components/logo"
import { SelectBox } from './selectbox'
import { Card } from "@/components/ui/card"
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";


interface AppSidebarProps {
  isConnected: boolean;
  handleConnectClick: () => void;
}

export function AppSidebar({ isConnected, handleConnectClick }: AppSidebarProps) {
  const [portList, setPortList] = useState([]);
  const baudRates = [110, 300, 600, 1200, 2400, 4800, 9600, 14400, 19200, 38400, 57600, 115200, 128000, 256000];

  async function listPorts() {
    let ports: any = await invoke("list_ports");
    ports = ports.map((port: any) => {
      return [port.port_name]
    });

    setPortList(ports);
  }

  useEffect(() => {
    listPorts();
  }, []);

  return (
    <Sidebar className="border-r border-black-600" >
      <SidebarHeader>
        <div style={{ position: 'relative', right: '10px', top: '0px' }}>
          <Logo className='pr-2' />
        </div>
        <div style={{ display: 'flex', alignItems: 'center', position: 'relative', left: '3px', top: '0px' }} className="text-lg font-light text-gray-400 px-6 montserrat-extralight" >
          <ChevronsLeft />
          <span>Signal Visualizer</span>
          <ChevronsRight />
          <hr />
        </div>
      </SidebarHeader>
      <SidebarContent>
        <SidebarMenu>
          <SidebarMenuItem className='mx-2 space-y-4'>
            <Card className='w-full'>
              <div className="flex flex-col w-full space-y-6 p-4">
                <div className="space-y-2">
                  <h1 className="text-center text-gray-400 montserrat-extralight">Wired</h1>
                  <p className="text-sm font-medium text-gray-400">Select a COM Port</p>
                  <div className="flex items-center space-x-2">
                    <SelectBox
                      options={portList}
                      placeholder="Select a port"
                      className="flex-grow"
                    />
                    <Button
                      variant="outline"
                      size="icon"
                      className="border border-gray-800"
                      onClick={() => listPorts()}
                    >
                      <RotateCcw className="h-4 w-4" />
                    </Button>
                  </div>
                </div>

                <div className="space-y-2">
                  <p className="text-sm font-medium text-gray-400">Select a Baud Rate</p>
                  <SelectBox
                    options={baudRates}
                    placeholder="Select a baud rate"
                    className="w-full"
                  />
                </div>
                <Button
                  variant="outline"
                  className="border border-gray-800 w-full mt-2"
                >
                  Connect
                </Button>
              </div>
              <hr />
              <div className="flex flex-col w-full space-y-6 p-4">
                <h1 className="text-center text-gray-400 montserrat-extralight">Wireless</h1>
                <div className="space-y-2">
                  <div className="flex items-center justify-between space-x-2">
                    <p className="text-sm font-medium text-gray-400">Connection Status</p>
                    {isConnected ? (
                      <Wifi className="h-3 w-4 items-right align-end" color="#6dd1c5" />
                    ) : (
                      <Unplug className="h-4 w-4 items-right align-end" color="#e21d48" />
                    )
                    }
                  </div>
                </div>

                <Button
                  variant="outline"
                  className="border border-gray-800 w-full mt-2"
                  onClick={handleConnectClick}
                >
                  {isConnected ? "Disconnect" : "Connect"}
                </Button>
              </div>
            </Card>
            <Card className='w-full'>
              <div className="flex flex-col w-full space-y-4 p-4">
                <p className='text-xl font-bold montserrat-extralight'>Instructions</p>
                <ol className="text-sm font-medium text-gray-400 list-decimal pl-6">
                  <li>Attach the EMG sensor firmly on the skin over the muscle group you want to monitor.</li>
                  <li>Choose the correct COM port and baud rate for communication.</li>
                  <li>Click Start to begin real-time signal visualization.</li>
                </ol>
              </div>
            </Card>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarContent>
    </Sidebar>
  )
}