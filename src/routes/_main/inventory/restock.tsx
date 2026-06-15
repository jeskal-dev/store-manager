import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_main/inventory/restock")({
  component: RouteComponent,
});

function RouteComponent() {
  return <RestockPage />;
}

function RestockPage() {
  return (
    <div className="rise-in">
      <div className="mb-6">
        <h1 className="display-title text-2xl font-bold text-[var(--sea-ink)]">
          Reposición
        </h1>
        <p className="mt-1 text-sm" style={{ color: "var(--sea-ink-soft)" }}>
          Registrar entrada de stock a una tienda
        </p>
      </div>
      <div className="island-shell rounded-xl p-8 text-center">
        <p className="text-sm" style={{ color: "var(--sea-ink-soft)" }}>
          Próximamente — Formulario de reposición de inventario
        </p>
      </div>
    </div>
  );
}
