import { createFileRoute } from "@tanstack/react-router";
import { WipPage } from "@/components/shared/wip-page";

export const Route = createFileRoute("/_main/purchases/")({
  component: () => (
    <WipPage
      title="Compras"
      description="Todas las órdenes de compra"
      sections={[
        { type: "filter-bar" },
        { type: "cards", count: 3 },
        { type: "table", rows: 10, cols: 5 },
      ]}
    />
  ),
});
