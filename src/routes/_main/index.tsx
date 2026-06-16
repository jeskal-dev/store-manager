import { createFileRoute } from "@tanstack/react-router";
import { useMemo, useState } from "react";
import { MetricCard } from "@/components/dashboard/metric-card";
import { SalesChart } from "@/components/dashboard/sales-chart";
import { StoreBreakdown } from "@/components/dashboard/store-breakdown";
import { TopProducts } from "@/components/dashboard/top-products";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Skeleton } from "@/components/ui/skeleton";
import { useDashboard } from "@/hooks/dashboard/use-dashboard";
import { formatDate, formatMoney, formatNumber } from "@/lib/utils";

export const Route = createFileRoute("/_main/")({
  component: DashboardPage,
});

const PERIOD_OPTIONS = [
  { value: "7", label: "7 días" },
  { value: "15", label: "15 días" },
  { value: "30", label: "30 días" },
  { value: "90", label: "90 días" },
];

function DashboardPage() {
  const [days, setDays] = useState("30");
  const { data, isLoading } = useDashboard({ days: Number.parseInt(days, 10) });

  const metrics = useMemo(() => {
    if (!data) return null;

    const { totalRevenue, totalSales } = data.salesOverTime.reduce(
      (sum, d) => {
        sum.totalRevenue += Number.parseFloat(d.total);
        sum.totalSales += d.count;
        return sum;
      },
      { totalRevenue: 0, totalSales: 0 },
    );
    const alertCount =
      data.inventorySummary.lowStock + data.inventorySummary.outOfStock;

    return { totalRevenue, totalSales, alertCount };
  }, [data]);

  if (isLoading) return <DashboardSkeleton />;

  if (!data) {
    return (
      <div className="flex h-64 flex-col items-center justify-center gap-2">
        <p className="text-xs text-muted-foreground">
          No hay datos para mostrar
        </p>
        <p className="text-[10px] text-muted-foreground/60">
          Verifica que existan ventas o movimientos registrados en el sistema
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3">
        <Select value={days} onValueChange={setDays}>
          <SelectTrigger size="sm">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {PERIOD_OPTIONS.map((opt) => (
              <SelectItem key={opt.value} value={opt.value}>
                {opt.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        <span className="text-[10px] text-muted-foreground">
          {formatDate(data.periodStart)} — {formatDate(data.periodEnd)}
        </span>
      </div>

      {/* Metric cards */}
      <div className="grid grid-cols-2 gap-3 md:grid-cols-4">
        <MetricCard
          title="Ventas totales"
          value={formatMoney(String(metrics?.totalRevenue ?? 0))}
          subtitle={`${formatNumber(metrics?.totalSales ?? 0)} transacciones`}
        />
        <MetricCard
          title="Productos"
          value={formatNumber(data.inventorySummary.totalProducts)}
          subtitle="En catálogo"
        />
        <MetricCard
          title="Alerta stock"
          value={formatNumber(metrics?.alertCount ?? 0)}
          subtitle={`${data.inventorySummary.lowStock} bajo · ${data.inventorySummary.outOfStock} agotado`}
          alert={(metrics?.alertCount ?? 0) > 0}
        />
        <MetricCard
          title="Inventario"
          value={formatMoney(data.inventorySummary.totalValue)}
          subtitle="Valor total"
        />
      </div>

      {/* Sales chart */}
      <SalesChart data={data.salesOverTime} />

      {/* Bottom section: top products + breakdown */}
      <div className="grid gap-6 md:grid-cols-2">
        <div className="rounded-lg border p-4">
          <h3 className="font-heading mb-4 text-sm font-semibold tracking-tight">
            Productos más vendidos
          </h3>
          <TopProducts products={data.topProducts} />
        </div>
        <div className="rounded-lg border p-4">
          <h3 className="font-heading mb-4 text-sm font-semibold tracking-tight">
            Desglose de ventas
          </h3>
          <StoreBreakdown
            byStore={data.salesByStore}
            byPayment={data.salesByPayment}
          />
        </div>
      </div>
    </div>
  );
}

function DashboardSkeleton() {
  return (
    <div className="space-y-4">
      <Skeleton className="h-7 w-32" />
      <div className="grid grid-cols-2 gap-3 md:grid-cols-4">
        {Array.from({ length: 4 }).map((_, i) => (
          <Skeleton key={i} className="h-24" />
        ))}
      </div>
      <Skeleton className="h-62" />
      <div className="grid gap-6 md:grid-cols-2">
        <Skeleton className="h-64" />
        <Skeleton className="h-64" />
      </div>
    </div>
  );
}
