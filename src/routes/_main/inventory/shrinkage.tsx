import { createFileRoute } from "@tanstack/react-router";
import { WipPage } from "@/components/shared/wip-page";

export const Route = createFileRoute("/_main/inventory/shrinkage")({
  component: () => (
    <WipPage
      title="Merma"
      description="Registrar pérdida o ajuste de inventario"
      sections={[
        { type: "form", fields: 4 },
        { type: "table", rows: 3, cols: 4 },
      ]}
    />
  ),
});
