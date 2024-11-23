
interface Customer {
    awaitingService: boolean;
    coordX: number;
    coordY: number;
    destinationX: number;
    destinationY: number;
    id: string;
}

interface Vehicle {
    activeTime: string | null;
    coordX: number;
    coordY: number;
    customerId: string | null;
    distanceTravelled: number | null;
    id: string;
    isAvailable: boolean;
    numberOfTrips: number | null;
    remainingTravelTime: number | null;
    vehicleSpeed: number | null;
}

const enum MessageType {
    INITIAL_STATE = "INITIAL_STATE",
    VEHICLE_UPDATE = "VEHICLE_UPDATE",
    PERSON_UPDATE = "PERSON_UPDATE",
}
interface InitialStateMessage {
    type: MessageType.INITIAL_STATE;
    id: string | null;
    startTime: string | null;
    endTime: string | null;
    status: string | null;
    customers: Customer[];
    vehicles: Vehicle[];
}

interface VehicleUpdateMessage {
    type: MessageType.VEHICLE_UPDATE;
    vehicle: Vehicle;
}

interface CustomerUpdateMessage {
    type: MessageType.PERSON_UPDATE;
    customer: Customer;
}

type WebSocketMessage = InitialStateMessage | VehicleUpdateMessage | CustomerUpdateMessage;

export {
    Customer,
    CustomerUpdateMessage,
    Vehicle,
    VehicleUpdateMessage,
    MessageType,
    InitialStateMessage,
    VehicleUpdateMessage,
    CustomerUpdateMessage,
    WebSocketMessage
}