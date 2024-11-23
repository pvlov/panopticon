import React, {createContext, useContext, useEffect, useState} from "react";
import { WEBSOCKET_URL } from "@/config";
import { InitialStateMessage, MessageType, WebSocketMessage, Vehicle, Customer , CustomerUpdateMessage, VehicleUpdateMessage } from "@/model/message.ts";

interface MessageOut {
    content: string;
}

interface WebSocketContextType {
    vehicles: Map<string, Vehicle>;
    customers: Map<string, Customer>;
    sendMessage: (message: MessageOut) => void;
}

const WebSocketContext = createContext<WebSocketContextType | undefined>(undefined);

export const WebSocketProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    const [customers, setCustomers] = useState<Map<string, Customer>>(new Map());
    const [vehicles, setVehicles] = useState<Map<string, Vehicle>>(new Map());
    const [socket, setSocket] = useState<WebSocket | null>(null);

    useEffect(() => {
        const ws = new WebSocket(WEBSOCKET_URL);
        setSocket(ws);

        ws.onopen = () => {
            console.log("WebSocket connection established");
        };

        ws.onmessage = (event) => {
            const message: WebSocketMessage = JSON.parse(event.data) as WebSocketMessage;

            switch (message.type) {
                case MessageType.INITIAL_STATE: {

                    const initialStateMessage = message as InitialStateMessage;
                    for (const customer of initialStateMessage.customers) {
                        setCustomers((prevCustomers) => new Map(prevCustomers.set(customer.id, customer)));
                    }
                    for (const vehicle of initialStateMessage.vehicles) {
                        setVehicles((prevVehicles) => new Map(prevVehicles.set(vehicle.id, vehicle)));
                    }
                    break;
                }

                case MessageType.VEHICLE_UPDATE: {
                    const vehicleUpdateMessage = message as VehicleUpdateMessage;
                    setVehicles((prevVehicles) => new Map(prevVehicles.set(vehicleUpdateMessage.vehicle.id, vehicleUpdateMessage.vehicle)));

                    break;
                }

                case MessageType.PERSON_UPDATE: {
                    const customerUpdateMessage = message as CustomerUpdateMessage;
                    setCustomers((prevCustomers) => new Map(prevCustomers.set(customerUpdateMessage.customer.id, customerUpdateMessage.customer)));
                    break;
                }
            }
        };

        ws.onerror = (error) => {
            console.error("WebSocket error:", error);
        };

        ws.onclose = () => {
            console.log("WebSocket connection closed");
        };

        return () => {
            ws.close();
        };
    }, []);

    const sendMessage = (message: MessageOut) => {
        if (socket && socket.readyState === WebSocket.OPEN) {
            socket.send(JSON.stringify(message));
        }
    };

    return (
        <WebSocketContext.Provider value={{ customers, vehicles, sendMessage }}>
            {children}
        </WebSocketContext.Provider>
    );
};

export const useWebSocket = () => {
    const context = useContext(WebSocketContext);
    if (context === undefined) {
        throw new Error("useWebSocket must be used within a WebSocketProvider");
    }
    return context;
};