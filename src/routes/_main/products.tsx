import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_main/products")({
  component: RouteComponent,
});

function RouteComponent() {
  return <ProductsPage />;
}

function ProductsPage() {
  return (
    <div className="rise-in">
      <div className="mb-6">
        <h1 className="display-title text-2xl font-bold text-[var(--sea-ink)]">
          Productos
        </h1>
        <p className="mt-1 text-sm" style={{ color: "var(--sea-ink-soft)" }}>
          Catálogo de productos
        </p>
      </div>
      <div className="island-shell rounded-xl p-8 text-center">
        <p className="text-sm" style={{ color: "var(--sea-ink-soft)" }}>
          Próximamente — Gestión de productos
        </p>
      </div>
    </div>
  );
}
