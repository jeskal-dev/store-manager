import { createFileRoute } from "@tanstack/react-router";
import { WipPage } from "@/components/shared/wip-page";

export const Route = createFileRoute("/_main/supply-agreements")({
  component: () => (
    <WipPage
      title="Acuerdos"
      description="Convenios de suministro con proveedores"
      sections={[
        { type: "filter-bar" },
        { type: "table", rows: 6, cols: 4 },
      ]}
    />
  ),
});
