import { createFileRoute } from "@tanstack/react-router";
import { WipPage } from "@/components/shared/wip-page";

export const Route = createFileRoute("/_main/stores")({
  component: () => (
    <WipPage
      title="Tiendas"
      description="Gestiona las sucursales y puntos de venta"
      sections={[
        { type: "filter-bar" },
        { type: "cards", count: 3 },
        { type: "table", rows: 5, cols: 5 },
      ]}
    />
  ),
});
