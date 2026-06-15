import { createFileRoute } from "@tanstack/react-router";
import { useDashboard } from "@/hooks/dashboard/use-dashboard";
import { formatMoney, formatNumber } from "@/lib/utils";
import { Skeleton } from "@/components/ui/skeleton";
import {
  Area,
  AreaChart,
  Bar,
  BarChart,
  CartesianGrid,
  Cell,
  Pie,
  PieChart,
  XAxis,
  YAxis,
  ResponsiveContainer,
  Tooltip,
  Legend,
} from "recharts";

export const Route = createFileRoute("/_main/")({
  component: DashboardPage,
});

/* ───── KPI Card ───── */

function KpiCard({
  label,
  value,
  sublabel,
  loading,
}: {
  label: string;
  value: string;
  sublabel?: string;
  loading?: boolean;
}) {
  if (loading) {
    return (
      <div className="island-shell rounded-xl p-4 md:p-5">
        <Skeleton className="mb-2 h-3 w-20" />
        <Skeleton className="mb-1 h-7 w-28" />
        <Skeleton className="h-3 w-16" />
      </div>
    );
  }

  return (
    <div className="island-shell relative rounded-xl p-4 md:p-5">
      <span
        className="absolute right-3 top-3 block h-2 w-2 animate-pulse rounded-full"
        style={{
          backgroundColor: "var(--lagoon)",
          boxShadow: "0 0 6px var(--lagoon)",
        }}
      />
      <p
        className="mb-1 text-[0.69rem] font-bold uppercase tracking-[0.16em]"
        style={{ color: "var(--sea-ink-soft)" }}
      >
        {label}
      </p>
      <p
        className="display-title text-2xl font-bold md:text-3xl"
        style={{
          color: "var(--sea-ink)",
          fontVariantNumeric: "tabular-nums",
        }}
      >
        {value}
      </p>
      {sublabel && (
        <p className="mt-0.5 text-xs" style={{ color: "var(--sea-ink-soft)" }}>
          {sublabel}
        </p>
      )}
    </div>
  );
}

/* ───── Chart skeletons ───── */

function ChartSkeleton({ className }: { className?: string }) {
  return (
    <div className={`island-shell rounded-xl p-4 md:p-5 ${className ?? ""}`}>
      <Skeleton className="mb-1 h-4 w-32" />
      <Skeleton className="mb-4 h-3 w-24" />
      <Skeleton className="h-48 w-full rounded-lg" />
    </div>
  );
}

/* ───── Error state ───── */

function DashboardError({
  message,
  onRetry,
}: {
  message: string;
  onRetry: () => void;
}) {
  return (
    <div className="rise-in island-shell mx-auto mt-12 max-w-md rounded-xl p-8 text-center">
      <div
        className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full"
        style={{ backgroundColor: "rgba(212, 149, 43, 0.15)" }}
      >
        <span className="text-xl">⚠</span>
      </div>
      <h2 className="display-title mb-2 text-lg font-bold text-[var(--sea-ink)]">
        No pudimos cargar el tablero
      </h2>
      <p className="mb-5 text-sm" style={{ color: "var(--sea-ink-soft)" }}>
        {message}
      </p>
      <button
        type="button"
        onClick={onRetry}
        className="inline-flex cursor-pointer items-center gap-2 rounded-lg px-4 py-2 text-sm font-medium text-white transition-colors"
        style={{ backgroundColor: "var(--lagoon-deep)" }}
        onMouseEnter={(e) =>
          (e.currentTarget.style.backgroundColor = "var(--lagoon)")
        }
        onMouseLeave={(e) =>
          (e.currentTarget.style.backgroundColor = "var(--lagoon-deep)")
        }
      >
        Reintentar
      </button>
    </div>
  );
}

/* ───── Empty state ───── */

function DashboardEmpty() {
  return (
    <div className="rise-in island-shell mx-auto mt-12 max-w-md rounded-xl p-8 text-center">
      <div
        className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full"
        style={{ backgroundColor: "rgba(79, 184, 178, 0.15)" }}
      >
        <span className="text-xl">📊</span>
      </div>
      <h2 className="display-title mb-2 text-lg font-bold text-[var(--sea-ink)]">
        No hay datos aún
      </h2>
      <p
        className="text-sm leading-relaxed"
        style={{ color: "var(--sea-ink-soft)" }}
      >
        El tablero mostrará ingresos, productos más vendidos y resumen de
        inventario cuando comiences a registrar ventas y stock.
      </p>
    </div>
  );
}

/* ───── Custom Tooltip ───── */

function DashTooltip({ active, payload, label }: any) {
  if (!active || !payload?.length) return null;
  return (
    <div className="rounded-lg border border-[var(--line)] bg-[var(--surface-strong)] px-3 py-2 text-xs shadow-xl backdrop-blur-md">
      <p className="mb-1 font-medium text-[var(--sea-ink)]">{label}</p>
      {payload.map((entry: any, i: number) => (
        <p key={i} style={{ color: entry.color }}>
          {entry.name}: {formatMoney(Number(entry.value).toFixed(2))}
        </p>
      ))}
    </div>
  );
}

/* ───── Main component ───── */

function DashboardPage() {
  const { data, isLoading, isError, error, refetch } = useDashboard();

  if (isLoading) {
    return (
      <div className="rise-in">
        <Skeleton className="mb-1 h-7 w-28" />
        <Skeleton className="mb-6 h-4 w-44" />
        <div className="mb-6 grid grid-cols-2 gap-3 md:grid-cols-4 md:gap-4">
          <KpiCard label="Cargando" value="" loading />
          <KpiCard label="Cargando" value="" loading />
          <KpiCard label="Cargando" value="" loading />
          <KpiCard label="Cargando" value="" loading />
        </div>
        <ChartSkeleton />
        <div className="mt-6 grid gap-6 md:grid-cols-2">
          <ChartSkeleton />
          <ChartSkeleton />
        </div>
        <ChartSkeleton className="mt-6" />
      </div>
    );
  }

  if (isError) {
    return (
      <DashboardError
        message={error?.message ?? "Error desconocido"}
        onRetry={() => refetch()}
      />
    );
  }

  if (!data) return null;

  const hasSales = data.sales_over_time.length > 0;
  const hasProducts = data.top_products.length > 0;
  const hasStores = data.sales_by_store.length > 0;
  const hasPayments = data.sales_by_payment.length > 0;

  if (!hasSales && !hasProducts && !hasStores && !hasPayments) {
    return <DashboardEmpty />;
  }

  const totalRevenue = data.sales_over_time.reduce(
    (acc, d) => acc + Number.parseFloat(d.total),
    0,
  );
  const totalSalesCount = data.sales_over_time.reduce(
    (acc, d) => acc + d.count,
    0,
  );
  const inv = data.inventory_summary;

  return (
    <div className="rise-in">
      {/* Page header */}
      <div className="mb-6">
        <h1 className="display-title text-2xl font-bold text-[var(--sea-ink)]">
          Tablero
        </h1>
        <p className="mt-1 text-sm" style={{ color: "var(--sea-ink-soft)" }}>
          Resumen · Últimos {data.period_start} a {data.period_end}
        </p>
      </div>

      {/* KPI cards */}
      <div className="mb-6 grid grid-cols-2 gap-3 md:grid-cols-4 md:gap-4">
        <KpiCard
          label="Ingresos"
          value={formatMoney(totalRevenue.toFixed(2))}
          sublabel={`${formatNumber(totalSalesCount)} ventas`}
        />
        <KpiCard
          label="Ventas"
          value={formatNumber(totalSalesCount)}
          sublabel="totales en el período"
        />
        <KpiCard
          label="Stock bajo"
          value={formatNumber(inv.low_stock)}
          sublabel={`${formatNumber(inv.out_of_stock)} agotados`}
        />
        <KpiCard
          label="Valor inventario"
          value={formatMoney(inv.total_value)}
          sublabel={`${formatNumber(inv.total_products)} productos`}
        />
      </div>

      {/* Sales over time */}
      {hasSales && <SalesOverTimeChart sales={data.sales_over_time as any} />}

      {/* Bottom row */}
      <div className="mt-6 grid gap-6 md:grid-cols-2">
        {hasProducts && <TopProductsChart products={data.top_products as any} />}
        {hasPayments && <PaymentChart payments={data.sales_by_payment as any} />}
      </div>

      {/* Sales by store */}
      {hasStores && <StoreChart stores={data.sales_by_store as any} />}
    </div>
  );
}

/* ───── Sales Over Time ───── */

function SalesOverTimeChart({
  sales,
}: {
  sales: { date: string; total: string; count: number }[];
}) {
  const data = sales.map((d) => ({
    date: String(d.date),
    total: Number.parseFloat(d.total),
  }));

  return (
    <div className="island-shell rounded-xl p-4 md:p-5">
      <h3 className="display-title text-base font-bold text-[var(--sea-ink)]">
        Ventas en el tiempo
      </h3>
      <p className="mb-4 text-xs" style={{ color: "var(--sea-ink-soft)" }}>
        Ingresos diarios · últimos 30 días
      </p>
      <div className="h-64 w-full">
        <ResponsiveContainer width="100%" height="100%">
          <AreaChart data={data}>
            <defs>
              <linearGradient id="salesGrad" x1="0" y1="0" x2="0" y2="1">
                <stop
                  offset="0%"
                  stopColor="var(--lagoon)"
                  stopOpacity={0.35}
                />
                <stop
                  offset="100%"
                  stopColor="var(--lagoon)"
                  stopOpacity={0.02}
                />
              </linearGradient>
            </defs>
            <CartesianGrid
              strokeDasharray="3 3"
              stroke="var(--line)"
              vertical={false}
            />
            <XAxis
              dataKey="date"
              tickLine={false}
              axisLine={false}
              tick={{ fontSize: 11, fill: "var(--sea-ink-soft)" }}
              tickFormatter={(v: string) => {
                const d = new Date(v);
                return `${d.getDate()}/${d.getMonth() + 1}`;
              }}
            />
            <YAxis
              tickLine={false}
              axisLine={false}
              tick={{ fontSize: 11, fill: "var(--sea-ink-soft)" }}
              tickFormatter={(v: number) =>
                v >= 1000 ? `$${(v / 1000).toFixed(0)}k` : `$${v}`
              }
            />
            <Tooltip content={<DashTooltip />} />
            <Area
              type="monotone"
              dataKey="total"
              name="Ingresos"
              stroke="var(--lagoon)"
              strokeWidth={2}
              fill="url(#salesGrad)"
            />
          </AreaChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}

/* ───── Top Products ───── */

function TopProductsChart({
  products,
}: {
  products: { name: string; total_revenue: string; units_sold: number }[];
}) {
  const data = products.map((d) => ({
    name: d.name,
    revenue: Number.parseFloat(d.total_revenue),
  }));

  return (
    <div className="island-shell rounded-xl p-4 md:p-5">
      <h3 className="display-title mb-4 text-base font-bold text-[var(--sea-ink)]">
        Productos más vendidos
      </h3>
      <div className="h-64 w-full">
        <ResponsiveContainer width="100%" height="100%">
          <BarChart data={data} layout="vertical">
            <CartesianGrid
              strokeDasharray="3 3"
              stroke="var(--line)"
              horizontal={false}
            />
            <XAxis
              type="number"
              tickLine={false}
              axisLine={false}
              tick={{ fontSize: 11, fill: "var(--sea-ink-soft)" }}
              tickFormatter={(v: number) => `$${(v / 1000).toFixed(0)}k`}
            />
            <YAxis
              type="category"
              dataKey="name"
              tickLine={false}
              axisLine={false}
              tick={{ fontSize: 11, fill: "var(--sea-ink)" }}
              width={110}
            />
            <Tooltip content={<DashTooltip />} />
            <Bar
              dataKey="revenue"
              name="Ingresos"
              fill="var(--lagoon)"
              radius={[0, 4, 4, 0]}
              barSize={20}
            />
          </BarChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}

/* ───── Payment Methods ───── */

const COLORS = [
  "var(--lagoon)",
  "var(--lagoon-deep)",
  "var(--palm)",
  "#d4952b",
  "#b8708a",
];

function PaymentChart({
  payments,
}: {
  payments: { method: string; total_revenue: string; count: number }[];
}) {
  const data = payments.map((d, i) => ({
    name: d.method,
    value: Number.parseFloat(d.total_revenue),
    fill: COLORS[i % COLORS.length],
  }));

  return (
    <div className="island-shell rounded-xl p-4 md:p-5">
      <h3 className="display-title mb-4 text-base font-bold text-[var(--sea-ink)]">
        Métodos de pago
      </h3>
      <div className="h-64 w-full">
        <ResponsiveContainer width="100%" height="100%">
          <PieChart>
            <Pie
              data={data}
              dataKey="value"
              nameKey="name"
              cx="50%"
              cy="50%"
              innerRadius={55}
              outerRadius={85}
              paddingAngle={2}
            >
              {data.map((entry) => (
                <Cell key={entry.name} fill={entry.fill} />
              ))}
            </Pie>
            <Tooltip content={<DashTooltip />} />
            <Legend
              verticalAlign="bottom"
              iconType="circle"
              formatter={(value: string) => (
                <span style={{ color: "var(--sea-ink)", fontSize: 12 }}>
                  {value}
                </span>
              )}
            />
          </PieChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}

/* ───── Sales by Store ───── */

function StoreChart({
  stores,
}: {
  stores: { name: string; total_revenue: string; sales_count: number }[];
}) {
  const data = stores.map((d) => ({
    name: d.name,
    revenue: Number.parseFloat(d.total_revenue),
  }));

  return (
    <div className="island-shell mt-6 rounded-xl p-4 md:p-5">
      <h3 className="display-title text-base font-bold text-[var(--sea-ink)]">
        Ventas por tienda
      </h3>
      <p className="mb-4 text-xs" style={{ color: "var(--sea-ink-soft)" }}>
        Comparativa de ingresos entre locales
      </p>
      <div className="h-64 w-full">
        <ResponsiveContainer width="100%" height="100%">
          <BarChart data={data}>
            <CartesianGrid
              strokeDasharray="3 3"
              stroke="var(--line)"
              vertical={false}
            />
            <XAxis
              dataKey="name"
              tickLine={false}
              axisLine={false}
              tick={{ fontSize: 11, fill: "var(--sea-ink)" }}
            />
            <YAxis
              tickLine={false}
              axisLine={false}
              tick={{ fontSize: 11, fill: "var(--sea-ink-soft)" }}
              tickFormatter={(v: number) =>
                v >= 1000 ? `$${(v / 1000).toFixed(0)}k` : `$${v}`
              }
            />
            <Tooltip content={<DashTooltip />} />
            <Bar
              dataKey="revenue"
              name="Ingresos"
              fill="var(--lagoon)"
              radius={[4, 4, 0, 0]}
              barSize={40}
            />
          </BarChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}
