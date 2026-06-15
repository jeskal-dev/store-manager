import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_main/purchases/register")({
  component: RouteComponent,
});

function RouteComponent() {
  return <RegisterPurchasePage />;
}

function RegisterPurchasePage() {
  return (
    <div className="rise-in">
      <div className="mb-6">
        <h1 className="display-title text-2xl font-bold text-[var(--sea-ink)]">
          Registrar compra
        </h1>
        <p className="mt-1 text-sm" style={{ color: "var(--sea-ink-soft)" }}>
          Registra una nueva compra a proveedor
        </p>
      </div>
      <div className="island-shell rounded-xl p-8 text-center">
        <p className="text-sm" style={{ color: "var(--sea-ink-soft)" }}>
          Próximamente — Formulario de registro de compra
        </p>
      </div>
    </div>
  );
}
