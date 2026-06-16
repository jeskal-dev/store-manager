import { createFileRoute } from "@tanstack/react-router";
import { WipPage } from "@/components/shared/wip-page";

export const Route = createFileRoute("/_main/inventory/")({
  component: () => (
    <WipPage
      title="Inventario"
      description="Todos los items en inventario"
      sections={[
        { type: "filter-bar" },
        { type: "cards", count: 4 },
        { type: "table", rows: 10, cols: 6 },
      ]}
    />
  ),
});
