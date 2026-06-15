import { createFileRoute, Outlet } from "@tanstack/react-router";

export const Route = createFileRoute("/_main/inventory")({
  component: InventoryLayout,
});

function InventoryLayout() {
  return <Outlet />;
}
