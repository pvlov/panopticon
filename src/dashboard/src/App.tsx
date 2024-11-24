import Map from "@/components/map.tsx";
import TopBar from "@/components/top-bar.tsx";
import { PieChartCard } from "@/components/pie-chart.tsx";
import { ThemeProvider } from "@/components/theme-provider.tsx";
import './App.css'
import "leaflet/dist/leaflet.css";
import { RadialChart } from "@/components/radial-chart.tsx";
import {AppState, Vehicle, } from "@/model/models.ts";
import { BarChartCard} from "@/components/bar-chart.tsx";
import { ControlPanel } from "@/components/control-panel.tsx";
import {useEffect, useState} from "react";
import axios from "axios";
import {BASE_PATH, SCENARIO_ID, setScenarioId, setSolver, SOLVER} from "@/config.ts";
import SelectionScreen from "@/components/selection-screen.tsx";


function initScenario(scenarioId: string) {
    const url = `${BASE_PATH}/run_scenario/${scenarioId}/${SOLVER}/0.05`;

    axios.get(url)
        .then(response => console.log(response))
        .catch(error => console.error(error));
}

const useAppState = (interval: number = 100) => {
    const url = `${BASE_PATH}/get_current_scenario`;

    const [ appState, setAppState ] = useState<AppState | null>(null);
    useEffect(() => {
        const fetchData = async () => {
            try {
                const response = await axios.get(url);

                const totalDistance = (response.data.vehicles.reduce((acc: number, vehicle: Vehicle) => acc + (vehicle.distanceTravelled || 0), 0)) / 10000;
                const totalEnergyConsumption = 0.015 * totalDistance * 10;

                // make up a totaly bogus way rto calculate the profits using the above data
                // just to show how we can use the data
                const totalProfits = (totalDistance * 0.5) - totalEnergyConsumption * 0.1;

                const parsedData = {
                    customers: response.data.customers,
                    vehicles: response.data.vehicles,
                    vehicleMetrics: {
                        totalEnergyConsumption: totalEnergyConsumption, // kwh/m
                        totalDistance: totalDistance, // == km
                        totalProfits: totalProfits,
                        idleCount: response.data.vehicles.reduce((acc: number, vehicle: Vehicle) => acc + (vehicle.customerId === null ? 1 : 0), 0),
                        collectingCount: response.data.vehicles.reduce((acc: number, vehicle: Vehicle) => acc + (vehicle.vehicleSpeed > 0 ? 1 : 0), 0),
                        transportingCount: response.data.vehicles.reduce((acc: number, vehicle: Vehicle) => acc + (vehicle.customerId !== null ? 1 : 0), 0),
                    }
                } as AppState;

                setAppState(parsedData);
            } catch (error) {
                console.error("Error fetching data:", error);
            }
        }

        const intervalId = setInterval(fetchData, interval);

        return () => clearInterval(intervalId);
    }, [url, interval]);

    return appState;
}

function AppContent() {

    const appState = useAppState();

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
                            title="Key metrics"
                            subtitle="Important metrics on the current scenario"
                            appState={appState}
                        />
                        < ControlPanel />
                    </div>

                </div>
            </ThemeProvider>
    )
}
function App() {

    const [selectionMade, setSelectionMade] = useState(false);

    const handleSelection = (scenarioId: string, solver: string) => {
        setScenarioId(scenarioId);
        setSolver(solver);
        setSelectionMade(true);
        initScenario(scenarioId)
    }

    initScenario(SCENARIO_ID);

    return (
        <div>
            {selectionMade ? <AppContent /> : <SelectionScreen onSelection={handleSelection} />}
        </div>
    )
}

export default App
