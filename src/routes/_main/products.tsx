import { createFileRoute } from "@tanstack/react-router";
import { WipPage } from "@/components/shared/wip-page";

export const Route = createFileRoute("/_main/products")({
  component: () => (
    <WipPage
      title="Productos"
      description="Catálogo completo de productos"
      sections={[
        { type: "filter-bar" },
        { type: "cards", count: 3 },
        { type: "table", rows: 8, cols: 5 },
      ]}
    />
  ),
});
