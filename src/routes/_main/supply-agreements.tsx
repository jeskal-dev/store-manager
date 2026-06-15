import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_main/supply-agreements")({
  component: RouteComponent,
});

function RouteComponent() {
  return <SupplyAgreementsPage />;
}

function SupplyAgreementsPage() {
  return (
    <div className="rise-in">
      <div className="mb-6">
        <h1 className="display-title text-2xl font-bold text-[var(--sea-ink)]">
          Acuerdos de suministro
        </h1>
        <p className="mt-1 text-sm" style={{ color: "var(--sea-ink-soft)" }}>
          Acuerdos con proveedores por producto
        </p>
      </div>
      <div className="island-shell rounded-xl p-8 text-center">
        <p className="text-sm" style={{ color: "var(--sea-ink-soft)" }}>
          Próximamente — Gestión de acuerdos de suministro
        </p>
      </div>
    </div>
  );
}
