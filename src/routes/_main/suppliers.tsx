import { createFileRoute } from "@tanstack/react-router";
import { WipPage } from "@/components/shared/wip-page";

export const Route = createFileRoute("/_main/suppliers")({
  component: () => (
    <WipPage
      title="Proveedores"
      description="Directorio de proveedores activos"
      sections={[
        { type: "filter-bar" },
        { type: "table", rows: 6, cols: 5 },
      ]}
    />
  ),
});
