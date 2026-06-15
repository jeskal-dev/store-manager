import { createFileRoute, Outlet } from "@tanstack/react-router";

export const Route = createFileRoute("/_main/purchases")({
  component: PurchasesLayout,
});

function PurchasesLayout() {
  return <Outlet />;
}
