import { createFileRoute } from "@tanstack/react-router";
import { WipPage } from "@/components/shared/wip-page";

export const Route = createFileRoute("/_main/sales/register")({
  component: () => (
    <WipPage
      title="Registrar venta"
      description="Crear una nueva transacción de venta"
      sections={[
        { type: "form", fields: 6 },
        { type: "table", rows: 3, cols: 4 },
      ]}
    />
  ),
});
