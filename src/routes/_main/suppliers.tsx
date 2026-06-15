import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_main/suppliers")({
  component: RouteComponent,
});

function RouteComponent() {
  return <SuppliersPage />;
}

function SuppliersPage() {
  return (
    <div className="rise-in">
      <div className="mb-6">
        <h1 className="display-title text-2xl font-bold text-[var(--sea-ink)]">
          Proveedores
        </h1>
        <p className="mt-1 text-sm" style={{ color: "var(--sea-ink-soft)" }}>
          Gestiona tus proveedores
        </p>
      </div>
      <div className="island-shell rounded-xl p-8 text-center">
        <p className="text-sm" style={{ color: "var(--sea-ink-soft)" }}>
          Próximamente — Gestión de proveedores
        </p>
      </div>
    </div>
  );
}
