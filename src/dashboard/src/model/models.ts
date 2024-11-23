
interface AppState {
    customers: Customer[];
    vehicles: Vehicle[];
    vehicleMetrics: VehicleMetrics;
}

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

interface VehicleMetrics {
    totalDistance: number;
    idleCount: number;
    collectingCount: number;
    transportingCount: number;
}

export {
    VehicleMetrics,
    Customer,
    Vehicle,
    AppState
}