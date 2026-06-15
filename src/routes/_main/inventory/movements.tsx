import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_main/inventory/movements")({
  component: RouteComponent,
});

function RouteComponent() {
  return <MovementsPage />;
}

function MovementsPage() {
  return (
    <div className="rise-in">
      <div className="mb-6">
        <h1 className="display-title text-2xl font-bold text-[var(--sea-ink)]">
          Movimientos
        </h1>
        <p className="mt-1 text-sm" style={{ color: "var(--sea-ink-soft)" }}>
          Historial de movimientos de inventario
        </p>
      </div>
      <div className="island-shell rounded-xl p-8 text-center">
        <p className="text-sm" style={{ color: "var(--sea-ink-soft)" }}>
          Próximamente — Historial de movimientos de stock
        </p>
      </div>
    </div>
  );
}
