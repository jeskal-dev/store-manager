import { useMemo } from "react";
import { Area, AreaChart, CartesianGrid, XAxis, YAxis } from "recharts";
import type { DailySalesOutput } from "@/client/types";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from "@/components/ui/chart";
import { formatDate, formatMoney, formatNumber } from "@/lib/utils";

type SalesChartProps = {
  data: DailySalesOutput[];
};

const chartConfig = {
  total: {
    label: "Ventas",
    color: "var(--chart-1)",
  },
};

export function SalesChart({ data }: SalesChartProps) {
  const chartData = useMemo(
    () =>
      data.map((d) => ({
        date: formatDate(d.date, {
          day: "numeric",
          month: "short",
        }),
        total: Number.parseFloat(d.total),
        count: d.count,
      })),
    [data],
  );

  return (
    <Card>
      <CardHeader className="pb-0">
        <CardTitle className="text-xs font-medium">
          Ventas en el tiempo
        </CardTitle>
      </CardHeader>
      <CardContent className="pt-4">
        <ChartContainer config={chartConfig} className="h-50 w-full">
          <AreaChart data={chartData}>
            <CartesianGrid
              vertical={false}
              stroke="var(--border)"
              strokeDasharray="3 3"
            />
            <XAxis
              dataKey="date"
              tickLine={false}
              axisLine={false}
              tickMargin={8}
              fontSize={10}
              stroke="var(--muted-foreground)"
            />
            <YAxis
              tickLine={false}
              axisLine={false}
              tickMargin={8}
              fontSize={10}
              stroke="var(--muted-foreground)"
              tickFormatter={(v: number) =>
                formatNumber(v, {
                  notation: "compact",
                  compactDisplay: "short",
                })
              }
            />
            <ChartTooltip
              content={
                <ChartTooltipContent
                  formatter={(value) => formatMoney(String(value))}
                />
              }
            />
            <Area
              type="monotone"
              dataKey="total"
              stroke="var(--chart-1)"
              fill="var(--chart-1)"
              fillOpacity={0.15}
              strokeWidth={1.5}
            />
          </AreaChart>
        </ChartContainer>
      </CardContent>
    </Card>
  );
}
