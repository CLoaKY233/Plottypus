"use client"

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
} from "@/components/ui/card"
import {
  ChartConfig,
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from "@/components/ui/chart"
import { Area, AreaChart, CartesianGrid, XAxis, YAxis } from "recharts"

const chartConfig = {
  visitors: {
    label: "Visitors",
  },
  channel1: {
    label: "Channel 1",
    color: "hsl(var(--chart-1))",
  },
  channel2: {
    label: "Channel 2",
    color: "hsl(var(--chart-2))",
  },
} satisfies ChartConfig

type GraphComponentProps = {
  title?: string;
  blue?: boolean;
  green?: boolean;
  data?: Object[];
  minY?: number;
  maxY?: number;
};

export function GraphComponent({ 
  title, 
  blue = false, 
  green = false, 
  data = [], 
  minY = 0, 
  maxY = 100
}: GraphComponentProps) {
  return (
    <Card className="border">
      <CardHeader className="flex items-center gap-2 space-y-0 border-b py-5 sm:flex-row">
        <div>
          <CardDescription className="text-center">
            {title}
          </CardDescription>
        </div>
      </CardHeader>
      <CardContent className="px-2 pt-4 sm:px-6 sm:pt-6">
        <ChartContainer
          config={chartConfig}
          className="aspect-auto h-[250px] w-full"
        >
          <AreaChart 
            data={data}
            margin={{ top: 10, right: 30, left: 0, bottom: 0 }}
          >
            <defs>
              <linearGradient id="fillChannel1" x1="0" y1="0" x2="0" y2="1">
                <stop
                  offset="5%"
                  stopColor="var(--color-channel1)"
                  stopOpacity={0.8}
                />
                <stop
                  offset="95%"
                  stopColor="var(--color-channel1)"
                  stopOpacity={0.1}
                />
              </linearGradient>
              <linearGradient id="fillChannel2" x1="0" y1="0" x2="0" y2="1">
                <stop
                  offset="5%"
                  stopColor="var(--color-channel2)"
                  stopOpacity={0.8}
                />
                <stop
                  offset="95%"
                  stopColor="var(--color-channel2)"
                  stopOpacity={0.1}
                />
              </linearGradient>
            </defs>
            <CartesianGrid vertical={false} />
            <XAxis
              dataKey="timestamp"
              tickLine={false}
              axisLine={false}
              tickMargin={8}
              minTickGap={32}
              tickFormatter={(value) => {
                return value
              }}
            />
            <YAxis 
              domain={[minY, maxY]} 
              hide={true} 
            />
            <ChartTooltip
              cursor={false}
              content={
                <ChartTooltipContent
                  labelFormatter={(value) => {
                    return value
                  }}
                  indicator="dot"
                />
              }
            />
            {blue && (
              <Area
                dataKey="channel1"
                type="natural"
                fill="url(#fillChannel1)"
                stroke="var(--color-channel1)"
                stackId="a"
              />
            )}
            {green && (
              <Area
                dataKey="channel2"
                type="natural"
                fill="url(#fillChannel2)"
                stroke="var(--color-channel2)"
                stackId="a"
              />
            )}
          </AreaChart>
        </ChartContainer>
      </CardContent>
    </Card>
  )
}