import { useMemo } from "react";
import { Cell, Pie, PieChart } from "recharts";
import type {
  PaymentMethodSalesOutput,
  StoreSalesOutput,
} from "@/client/types";
import { ChartContainer, ChartTooltip } from "@/components/ui/chart";
import { getPaymentMethodLabel } from "@/constants/payment-methods";
import { formatMoney, formatNumber } from "@/lib/utils";

type StoreBreakdownProps = {
  byStore: StoreSalesOutput[];
  byPayment: PaymentMethodSalesOutput[];
};

const CHART_COLORS = [
  "var(--chart-1)",
  "var(--chart-2)",
  "var(--chart-3)",
  "var(--chart-4)",
  "var(--chart-5)",
];

function StoreBars({ stores }: { stores: StoreSalesOutput[] }) {
  const maxRevenue = useMemo(
    () => Math.max(...stores.map((s) => Number.parseFloat(s.totalRevenue))),
    [stores],
  );

  const totalRevenue = useMemo(
    () => stores.reduce((sum, s) => sum + Number.parseFloat(s.totalRevenue), 0),
    [stores],
  );

  return (
    <div className="space-y-3">
      {stores.map((store) => {
        const revenue = Number.parseFloat(store.totalRevenue);
        const barWidth = (revenue / maxRevenue) * 100;
        const share = ((revenue / totalRevenue) * 100).toFixed(0);

        return (
          <div key={store.storeId}>
            <div className="mb-1 flex items-baseline justify-between text-sm">
              <span className="truncate font-medium">{store.name}</span>
              <span className="shrink-0 font-heading text-xs font-semibold tabular-nums tracking-tight text-muted-foreground">
                {formatMoney(store.totalRevenue)}
              </span>
            </div>
            <div className="flex items-center gap-2">
              <div className="h-2 flex-1 overflow-hidden rounded-full bg-muted">
                <div
                  className="h-full rounded-full bg-chart-1 transition-all duration-500"
                  style={{ width: `${barWidth}%` }}
                />
              </div>
              <span className="w-8 text-right font-mono text-[11px] tabular-nums text-muted-foreground/50">
                {share}%
              </span>
            </div>
            <span className="font-mono text-[11px] tabular-nums text-muted-foreground/40">
              {formatNumber(store.salesCount)} ventas
            </span>
          </div>
        );
      })}
    </div>
  );
}

function PaymentBreakdown({
  methods,
}: {
  methods: PaymentMethodSalesOutput[];
}) {
  const totalRevenue = useMemo(
    () =>
      methods.reduce((sum, m) => sum + Number.parseFloat(m.totalRevenue), 0),
    [methods],
  );

  const chartData = useMemo(
    () =>
      methods.map((m) => ({
        name: m.method.toLowerCase(),
        value: Number.parseFloat(m.totalRevenue),
        revenue: m.totalRevenue,
        count: m.count,
      })),
    [methods],
  );

  return (
    <div className="space-y-3">
      <ChartContainer
        config={Object.fromEntries(
          methods.map((m, i) => [
            m.method.toLowerCase(),
            {
              label: getPaymentMethodLabel(m.method),
              color: CHART_COLORS[i % CHART_COLORS.length],
            },
          ]),
        )}
        className="h-48 w-full"
      >
        <PieChart>
          <Pie
            data={chartData}
            dataKey="value"
            nameKey="name"
            cx="50%"
            cy="50%"
            innerRadius={52}
            outerRadius={80}
            strokeWidth={0}
          >
            {chartData.map((entry, i) => (
              <Cell
                key={entry.name}
                fill={CHART_COLORS[i % CHART_COLORS.length]}
              />
            ))}
          </Pie>
          <ChartTooltip
            content={({ active, payload }) => {
              if (!active || !payload?.length) return null;
              const d = payload[0].payload;
              const label = getPaymentMethodLabel(d.name);
              const pct = ((d.value / totalRevenue) * 100).toFixed(0);
              return (
                <div className="rounded-none border border-border/50 bg-background px-2.5 py-1.5 text-xs shadow-xl">
                  <p className="font-medium">{label}</p>
                  <p>
                    <span className="font-mono font-semibold tabular-nums text-foreground">
                      {formatMoney(d.revenue)}
                    </span>
                    <span className="ml-1 text-muted-foreground">({pct}%)</span>
                  </p>
                  <p className="text-muted-foreground">
                    {formatNumber(d.count)} transacciones
                  </p>
                </div>
              );
            }}
          />
        </PieChart>
      </ChartContainer>

      <div className="space-y-1.5">
        {chartData.map((d, i) => {
          const pct = ((d.value / totalRevenue) * 100).toFixed(0);
          return (
            <div
              key={d.name}
              className="flex items-center justify-between text-sm"
            >
              <span className="flex items-center gap-1.5">
                <span
                  className="inline-block size-2 rounded-full"
                  style={{
                    backgroundColor: CHART_COLORS[i % CHART_COLORS.length],
                  }}
                />
                <span>{getPaymentMethodLabel(d.name)}</span>
              </span>
              <span className="font-mono text-xs font-semibold tabular-nums tracking-tight text-muted-foreground">
                {formatMoney(d.revenue)}
                <span className="ml-1 font-mono text-[11px] font-normal text-muted-foreground/50">
                  {pct}%
                </span>
              </span>
            </div>
          );
        })}
      </div>
    </div>
  );
}

export function StoreBreakdown({ byStore, byPayment }: StoreBreakdownProps) {
  return (
    <div className="grid gap-6 md:grid-cols-2">
      <div>
        <h4 className="font-heading mb-3 text-xs font-semibold tracking-tight text-muted-foreground">
          Por tienda
        </h4>
        <StoreBars stores={byStore} />
      </div>
      <div>
        <h4 className="font-heading mb-3 text-xs font-semibold tracking-tight text-muted-foreground">
          Por método de pago
        </h4>
        <PaymentBreakdown methods={byPayment} />
      </div>
    </div>
  );
}
