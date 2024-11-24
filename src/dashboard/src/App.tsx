import Map from "@/components/map.tsx";
import TopBar from "@/components/top-bar.tsx";
import { PieChartCard } from "@/components/pie-chart.tsx";
import { ThemeProvider } from "@/components/theme-provider.tsx";
import './App.css'
import "leaflet/dist/leaflet.css";
import { RadialChart } from "@/components/radial-chart.tsx";
import { AppState, Customer, Vehicle, VehicleMetrics } from "@/model/models.ts";
import { BarChartCard} from "@/components/bar-chart.tsx";
import { ControlPanel } from "@/components/control-panel.tsx";
import {useEffect, useState} from "react";
import axios from "axios";
import { BASE_PATH, SCENARIO_ID } from "@/config.ts";

const exampleCustomers: Customer[] = [
    {
        "awaitingService": true,
        "coordX": 48.127373,
        "coordY": 11.619146,
        "destinationX": 48.13746,
        "destinationY": 11.537177,
        "id": "78b14bdf-6443-4aab-80c6-438bbb83bf59"
    },
    {
        "awaitingService": true,
        "coordX": 48.11786,
        "coordY": 11.619865,
        "destinationX": 48.158703,
        "destinationY": 11.64584,
        "id": "12ec731e-89e0-4425-8d93-e94dd841cb6c"
    },
    {
        "awaitingService": true,
        "coordX": 48.15715,
        "coordY": 11.633884,
        "destinationX": 48.127247,
        "destinationY": 11.557555,
        "id": "7aef5716-5fed-4422-b378-eb12c1687bd4"
    },
    {
        "awaitingService": true,
        "coordX": 48.132896,
        "coordY": 11.643167,
        "destinationX": 48.129215,
        "destinationY": 11.506492,
        "id": "725c60e9-ecd3-4011-a603-19d0b6bafbf7"
    },
    {
        "awaitingService": true,
        "coordX": 48.149803,
        "coordY": 11.541854,
        "destinationX": 48.13667,
        "destinationY": 11.534629,
        "id": "9d32b375-e2c6-4abd-8076-6caa92a9d2f5"
    },
    {
        "awaitingService": false,
        "coordX": 48.143806,
        "coordY": 11.544565,
        "destinationX": 48.123795,
        "destinationY": 11.542089,
        "id": "b92f9a3f-e012-454f-bb47-b2a802fd37ae"
    },
    {
        "awaitingService": true,
        "coordX": 48.159286,
        "coordY": 11.521507,
        "destinationX": 48.14663,
        "destinationY": 11.591945,
        "id": "fb502366-f0b6-4fb6-b8b1-ba83a69758a0"
    },
    {
        "awaitingService": true,
        "coordX": 48.150047,
        "coordY": 11.567614,
        "destinationX": 48.141933,
        "destinationY": 11.624489,
        "id": "dbe17822-790b-49c0-accb-8f7a9b30b60d"
    },
    {
        "awaitingService": true,
        "coordX": 48.148064,
        "coordY": 11.573927,
        "destinationX": 48.1291,
        "destinationY": 11.608194,
        "id": "b5176582-e99b-4386-9e1a-265a7d6ae819"
    },
    {
        "awaitingService": false,
        "coordX": 48.165127,
        "coordY": 11.509435,
        "destinationX": 48.147408,
        "destinationY": 11.549371,
        "id": "9099171e-405f-4a3d-8ff3-60b49a23b5b0"
    }
]

const exampleVehicle: Vehicle[] = [
        {
            "activeTime": null,
            "coordX": 48.12397,
            "coordY": 11.575905,
            "customerId": null,
            "distanceTravelled": null,
            "id": "75f66c49-9cfb-4dba-89a8-6818ccf6e2a7",
            "isAvailable": true,
            "numberOfTrips": null,
            "remainingTravelTime": null,
            "vehicleSpeed": null
        },
    {
        "activeTime": null,
        "coordX": 48.158844,
        "coordY": 11.629238,
        "customerId": null,
        "distanceTravelled": null,
        "id": "b3ef8bc7-01a3-4b4d-a016-3266ad05a163",
        "isAvailable": true,
        "numberOfTrips": null,
        "remainingTravelTime": null,
        "vehicleSpeed": null
    },
    {
        "activeTime": null,
        "coordX": 48.12232,
        "coordY": 11.562825,
        "customerId": null,
        "distanceTravelled": null,
        "id": "30341deb-c986-4f84-9d6e-7763d72b5d45",
        "isAvailable": true,
        "numberOfTrips": null,
        "remainingTravelTime": null,
        "vehicleSpeed": null
    },
    {
        "activeTime": null,
        "coordX": 48.129528,
        "coordY": 11.585539,
        "customerId": null,
        "distanceTravelled": null,
        "id": "58e3f520-64bc-40f2-b0cd-ea2d8d1a5874",
        "isAvailable": true,
        "numberOfTrips": null,
        "remainingTravelTime": null,
        "vehicleSpeed": null
    },
    {
        "activeTime": null,
        "coordX": 48.126038,
        "coordY": 11.623396,
        "customerId": null,
        "distanceTravelled": null,
        "id": "ed12b400-a356-4ea6-99c3-fa61ef76c11f",
        "isAvailable": true,
        "numberOfTrips": null,
        "remainingTravelTime": null,
        "vehicleSpeed": null
    }
]

const exampleVehicleMetrics: VehicleMetrics = {
        "totalDistance": 1000,
        "idleCount": 10,
        "collectingCount": 20,
        "transportingCount": 30
    }

async function initScenario(scenarioId: string) {
    const url = `${BASE_PATH}/${scenarioId}/random`;

    try {
        const response = await axios.get(url);
        console.log("Scenario initialized", response.data);
    } catch (error) {
        console.error("Error initializing scenario", error);
    }
}

const useAppState = (interval: number = 100) => {
    const url = `${BASE_PATH}/get_current_scenario`;

    const { appState, setAppState } = useState(null);
    useEffect(() => {
        const fetchData = async () => {
            try {
                const response = await axios.get(url);
                console.log(response)
                setAppState(response.data);
            } catch (error) {
                console.error("Error fetching data:", error);
            }
        }

        const intervalId = setInterval(fetchData, interval);

        return () => clearInterval(intervalId);
    }, [url, interval, setAppState]);

    return JSON.parse(appState);
}

function AppContent() {

    const appState  = useAppState();

    if (!appState) {
        return <h1>:hedge:...</h1>
    }

    const pieData = [
        { status: "idle", count: appState.vehicleMetrics.idleCount, fill: "var(--color-idle)" },
        { status: "picking", count: appState.vehicleMetrics.collectingCount, fill: "var(--color-picking)" },
        { status: "transporting", count: appState.vehicleMetrics.transportingCount, fill: "var(--color-transporting)" },
    ]

    return (
            <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
                < TopBar />
                <div className="main-container">
                    <div className="map-container">
                        < Map customers={appState.customers} vehicles={appState.vehicles} />
                    </div>
                    <div className="charts-container">
                        < RadialChart customers={appState.customers} />
                        < PieChartCard
                            title="RoboTaxi Status"
                            subtitle="The current status of all RoboTaxis"
                            data={pieData}
                        />
                        < BarChartCard
                            title="Total Distance"
                            subtitle="Total distance travelled by all RoboTaxis"
                            appState={appState}
                        />
                        < ControlPanel />
                    </div>

                </div>
            </ThemeProvider>
    )
}
function App() {

    useEffect(() => {
        const initialize = async () => {
            await initScenario("");
        }
        initialize();
    }, []);

    return (
        <AppContent />
    )
}

export default App
