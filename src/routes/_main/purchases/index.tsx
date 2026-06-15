import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_main/purchases/")({
  component: RouteComponent,
});

function RouteComponent() {
  return <PurchasesPage />;
}

function PurchasesPage() {
  return (
    <div className="rise-in">
      <div className="mb-6">
        <h1 className="display-title text-2xl font-bold text-[var(--sea-ink)]">
          Compras
        </h1>
        <p className="mt-1 text-sm" style={{ color: "var(--sea-ink-soft)" }}>
          Historial de compras a proveedores
        </p>
      </div>
      <div className="island-shell rounded-xl p-8 text-center">
        <p className="text-sm" style={{ color: "var(--sea-ink-soft)" }}>
          Próximamente — Listado completo de compras
        </p>
      </div>
    </div>
  );
}
