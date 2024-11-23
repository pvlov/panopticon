import Map from "@/components/map.tsx";
import TopBar from "@/components/top-bar.tsx";
import { LineChart } from "@/components/line-chart.tsx";
import { PieChartCard } from "@/components/pie-chart.tsx";
import { ThemeProvider } from "@/components/theme-provider.tsx";
import { WebSocketProvider } from "@/components/WebSocketContext.tsx";
import './App.css'
import "leaflet/dist/leaflet.css";

function App() {

  const pieData = [
      { status: "idle" , count: 100, fill: "hsl(var(--chart-1))" },
      { status: "picking" , count: 200, fill: "hsl(var(--chart-2))" },
      { status: "transporting" , count: 300, fill: "hsl(var(--chart-3))" },
  ]

    const lineData = [
        { category: "A", value: 186 },
        { category: "B", value: 305 },
        { category: "C", value: 237 },
        { category: "D", value: 73 },
        { category: "E", value: 209 },
        { category: "F", value: 214 },
    ]


  return (
      <WebSocketProvider>
          <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
              < TopBar />
              <div className="main-container">
                  <div className="map-container">
                      < Map/>
                  </div>
                  <div className="charts-container">
                      < LineChart data={lineData}/>
                      < PieChartCard
                        title="Vehicle Status"
                        subtitle="Current vehicle status"
                        data={pieData}
                      />
                      < LineChart data={lineData}/>
                      < LineChart data={lineData}/>
                  </div>
              </div>
          </ThemeProvider>
      </WebSocketProvider>
  )
}

export default App
