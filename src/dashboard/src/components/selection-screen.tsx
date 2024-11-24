import * as React from "react";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card.tsx";
import { Input } from "@/components/ui/input.tsx";
import { Button } from "@/components/ui/button.tsx";
import { Label } from "@/components/ui/label.tsx";
import { ThemeProvider } from "@/components/theme-provider.tsx";

interface SelectionScreenProps {
    onSelection: (scenarioId: string, solver: string) => void;
}

const SelectionScreen: React.FC<SelectionScreenProps> = ({ onSelection }) => {
    const [scenarioId, setScenarioId] = React.useState("");
    const [solver, setSolver] = React.useState("");

    const handleSelection = () => {
        onSelection(scenarioId, solver);
    };

    return (
        <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
            <Card className="p-4">
                <CardHeader>
                    <CardTitle>Select a Solver</CardTitle>
                </CardHeader>
                <CardContent>
                    <div className="flex flex-col space-y-4">
                        <div>
                            <Label htmlFor="scenarioId">Scenario ID</Label>
                            <Input
                                id="scenarioId"
                                type="text"
                                placeholder="Enter Scenario ID"
                                value={scenarioId}
                                onChange={(e) => setScenarioId(e.target.value)}
                                className="input input-primary"
                            />
                        </div>
                        <div>
                            <Label htmlFor="solver">Solver</Label>
                            <Input
                                id="solver"
                                type="text"
                                placeholder="Enter Solver"
                                value={solver}
                                onChange={(e) => setSolver(e.target.value)}
                                className="input input-primary"
                            />
                        </div>
                        <Button onClick={handleSelection} className="btn btn-primary">Submit</Button>
                    </div>
                </CardContent>
            </Card>
        </ThemeProvider>
    );
};

export default SelectionScreen;