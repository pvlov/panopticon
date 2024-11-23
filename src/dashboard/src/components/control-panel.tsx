import { Card, CardHeader, CardTitle } from "@/components/ui/card.tsx";
import { cn } from "@/lib/utils";
import { Slider } from "@/components/ui/slider";
import { Switch } from "@/components/ui/switch";

export function ControlPanel() {
    return (
        <Card className="p-4">
            <CardHeader>
                <CardTitle>Control panel</CardTitle>
            </CardHeader>
            <div className="flex items-center mb-4">
                <Switch id="toggle" className="mr-2" />
                <label htmlFor="toggle" className="text-sm">Enable wait-time heatmap</label>
            </div>
            <div className="flex items-center">
                <Slider
                    id="slider"
                    defaultValue={[50]}
                    max={100}
                    step={1}
                    className={cn("w-[60%]")}
                />
                <label htmlFor="slider" className="text-sm ml-2">Adjust opacity</label>
            </div>
        </Card>
    );
}