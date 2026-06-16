import { createFileRoute } from "@tanstack/react-router";
import { WipPage } from "@/components/shared/wip-page";

export const Route = createFileRoute("/_main/inventory/restock")({
  component: () => (
    <WipPage
      title="Reposición"
      description="Registrar entrada de mercancía"
      sections={[
        { type: "form", fields: 5 },
        { type: "table", rows: 3, cols: 4 },
      ]}
    />
  ),
});
