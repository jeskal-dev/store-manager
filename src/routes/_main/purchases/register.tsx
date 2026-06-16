import { createFileRoute } from "@tanstack/react-router";
import { WipPage } from "@/components/shared/wip-page";

export const Route = createFileRoute("/_main/purchases/register")({
  component: () => (
    <WipPage
      title="Registrar compra"
      description="Crear una nueva orden de compra"
      sections={[
        { type: "form", fields: 5 },
        { type: "table", rows: 3, cols: 4 },
      ]}
    />
  ),
});
