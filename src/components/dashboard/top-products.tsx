import { useMemo } from "react";
import type { TopProductOutput } from "@/client/types";
import { cn, formatMoney, formatNumber } from "@/lib/utils";

type TopProductsProps = {
  products: TopProductOutput[];
};

export function TopProducts({ products }: TopProductsProps) {
  const maxRevenue = useMemo(
    () => Math.max(...products.map((p) => Number.parseFloat(p.totalRevenue))),
    [products],
  );

  return (
    <div className="divide-y divide-border">
      {products.map((product, i) => {
        const revenue = Number.parseFloat(product.totalRevenue);
        const barWidth = (revenue / maxRevenue) * 100;

        return (
          <div
            key={product.productId}
            className={cn(
              "group relative flex items-start gap-3 py-3 first:pt-0 last:pb-0",
              i === 0 && "py-4",
            )}
          >
            <span
              className={cn(
                "font-heading leading-none tracking-tight text-foreground",
                i === 0
                  ? "min-w-10 text-[2.5rem] font-bold"
                  : "min-w-8 text-2xl font-semibold text-foreground/60",
              )}
            >
              {i + 1}
            </span>

            <div className="min-w-0 flex-1">
              <div className="flex items-baseline justify-between gap-2">
                <div className="min-w-0 truncate">
                  <span className={cn("font-medium", i === 0 && "text-base")}>
                    {product.name}
                  </span>
                  <span className="ml-2 font-mono text-[11px] text-muted-foreground/60">
                    {product.productCode}
                  </span>
                </div>
                <span className="shrink-0 font-mono text-sm font-semibold tabular-nums tracking-tight">
                  {formatMoney(product.totalRevenue)}
                </span>
              </div>

              <div className="mt-1 flex items-center justify-between gap-2">
                <span className="font-mono text-[11px] tabular-nums text-muted-foreground/80">
                  {formatNumber(product.unitsSold)} uds.
                </span>
                <div className="h-1 w-full max-w-30 overflow-hidden rounded-full bg-muted">
                  <div
                    className="h-full rounded-full bg-chart-1 transition-all duration-500"
                    style={{ width: `${barWidth}%` }}
                  />
                </div>
              </div>
            </div>
          </div>
        );
      })}
    </div>
  );
}
