import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_main/sales/")({
  component: RouteComponent,
});

function RouteComponent() {
  return <SalesPage />;
}

function SalesPage() {
  return (
    <div className="rise-in">
      <div className="mb-6">
        <h1 className="display-title text-2xl font-bold text-[var(--sea-ink)]">
          Ventas
        </h1>
        <p className="mt-1 text-sm" style={{ color: "var(--sea-ink-soft)" }}>
          Historial de ventas realizadas
        </p>
      </div>
      <div className="island-shell rounded-xl p-8 text-center">
        <p className="text-sm" style={{ color: "var(--sea-ink-soft)" }}>
          Próximamente — Listado completo de ventas
        </p>
      </div>
    </div>
  );
}
