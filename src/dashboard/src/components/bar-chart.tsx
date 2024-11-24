import { TrendingUp } from "lucide-react";
import { Bar, BarChart, XAxis, YAxis } from "recharts";
import { AppState } from "@/model/models.ts";

import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from "@/components/ui/card";
import {
    ChartConfig,
    ChartContainer,
    ChartTooltip,
    ChartTooltipContent,
} from "@/components/ui/chart";

const chartConfig = {
    totalDistance: {
        label: "Total Distance",
        color: "hsl(var(--chart-3))",
    },
    totalEnergyConsumption: {
        label: "Total Energy Consumption",
        color: "hsl(var(--chart-4))",
    },
    totalProfits: {
        label: "Total Profits",
        color: "hsl(var(--chart-5))",
    },
} satisfies ChartConfig;

export function BarChartCard({ title, subtitle, appState }: { title: string; subtitle: string; appState: AppState }) {
    const chartData = [
        { metric: "totalDistance", value: appState.vehicleMetrics.totalDistance, fill: chartConfig.totalDistance.color },
        { metric: "totalEnergyConsumption", value: appState.vehicleMetrics.totalEnergyConsumption, fill: chartConfig.totalEnergyConsumption.color },
        { metric: "totalProfits", value: appState.vehicleMetrics.totalProfits, fill: chartConfig.totalProfits.color },
    ];

    return (
        <Card>
            <CardHeader>
                <CardTitle>{title}</CardTitle>
                <CardDescription>{subtitle}</CardDescription>
            </CardHeader>
            <CardContent>
                <ChartContainer config={chartConfig}>
                    <BarChart
                        accessibilityLayer
                        data={chartData}
                        layout="vertical"
                        margin={{
                            left: 0,
                        }}
                    >
                        <YAxis
                            dataKey="metric"
                            type="category"
                            tickLine={false}
                            tickMargin={10}
                            axisLine={false}
                            tickFormatter={(value) =>
                                chartConfig[value as keyof typeof chartConfig]?.label
                            }
                        />
                        <XAxis dataKey="value" type="number" hide />
                        <ChartTooltip
                            cursor={false}
                            content={<ChartTooltipContent hideLabel />}
                        />
                        <Bar dataKey="value" layout="vertical" radius={5} />
                    </BarChart>
                </ChartContainer>
            </CardContent>
            <CardFooter className="flex-col items-start gap-2 text-sm">
                <div className="flex gap-2 font-medium leading-none">
                    Profits up 24.6% this quarter<TrendingUp className="h-4 w-4" />
                </div>
            </CardFooter>
        </Card>
    );
}