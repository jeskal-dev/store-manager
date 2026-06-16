import { useLocation } from "@tanstack/react-router";
import { SidebarTrigger } from "../ui/sidebar";
import { ThemeToggle } from "./theme-toggle";

const ROUTE_TITLES: Record<string, string> = {
  "/": "Tablero",
  "/products": "Productos",
  "/stores": "Tiendas",
  "/suppliers": "Proveedores",
  "/supply-agreements": "Acuerdos",
  "/inventory": "Inventario",
  "/inventory/movements": "Movimientos",
  "/inventory/restock": "Reposición",
  "/inventory/shrinkage": "Merma",
  "/sales": "Ventas",
  "/sales/register": "Registrar venta",
  "/purchases": "Compras",
  "/purchases/register": "Registrar compra",
};

export function PageHeader() {
  const location = useLocation();
  const title = ROUTE_TITLES[location.pathname] ?? "Store Manager";

  return (
    <header className="flex h-12 items-center gap-2 border-b border-border px-4">
      <SidebarTrigger className="-ml-1 size-8" />
      <div className="mx-1 h-4 w-px bg-border" />
      <h1 className="font-heading text-sm font-semibold tracking-tight">{title}</h1>
      <div className="flex-1" />
      <ThemeToggle />
    </header>
  );
}
