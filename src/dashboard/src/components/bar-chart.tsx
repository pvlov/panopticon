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
        color: "hsl(var(--chart-1))",
    },
} satisfies ChartConfig;

export function BarChartCard({ title, subtitle, appState }: { title: string; subtitle: string; appState: AppState }) {
    const chartData = [
        { metric: "totalDistance", value: appState.vehicleMetrics.totalDistance, fill: chartConfig.totalDistance.color },
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
        </Card>
    );
}