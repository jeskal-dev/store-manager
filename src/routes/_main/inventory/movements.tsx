import { createFileRoute } from "@tanstack/react-router";
import { WipPage } from "@/components/shared/wip-page";

export const Route = createFileRoute("/_main/inventory/movements")({
  component: () => (
    <WipPage
      title="Movimientos"
      description="Historial de movimientos de inventario"
      sections={[
        { type: "filter-bar" },
        { type: "table", rows: 10, cols: 6 },
      ]}
    />
  ),
});
