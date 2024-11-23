import Map from "@/components/map.tsx";
import TopBar from "@/components/top-bar.tsx";
import { ThemeProvider } from "@/components/theme-provider.tsx";
import './App.css'
import "leaflet/dist/leaflet.css";

function App() {

  return (
      <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
          < TopBar />
          < Map />
      </ThemeProvider>
  )
}

export default App
