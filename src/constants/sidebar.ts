import {
  IconBuildingStore,
  IconFileDescription,
  IconLayoutDashboard,
  IconPackage,
  IconReceipt,
  IconShoppingCart,
  IconTags,
  IconTruck,
} from "@tabler/icons-react";
import type { SidebarSection } from "@/types/navigation";

export const SIDEBAR_NAV: SidebarSection[] = [
  {
    label: "General",
    links: [
      {
        label: "Tablero",
        url: "/",
        icon: IconLayoutDashboard,
      },
    ],
  },
  {
    label: "Inventario",
    links: [
      {
        label: "Inventario",
        icon: IconPackage,
        sublinks: [
          { label: "Todos los productos", url: "/inventory" },
          { label: "Movimientos", url: "/inventory/movements" },
          { label: "Reposición", url: "/inventory/restock" },
          { label: "Merma", url: "/inventory/shrinkage" },
        ],
      },
    ],
  },
  {
    label: "Ventas",
    links: [
      {
        label: "Ventas",
        icon: IconReceipt,
        sublinks: [
          { label: "Todas las ventas", url: "/sales" },
          { label: "Registrar venta", url: "/sales/register" },
        ],
      },
    ],
  },
  {
    label: "Compras",
    links: [
      {
        label: "Compras",
        icon: IconShoppingCart,
        sublinks: [
          { label: "Todas las compras", url: "/purchases" },
          { label: "Registrar compra", url: "/purchases/register" },
        ],
      },
    ],
  },
  {
    label: "Catálogo",
    links: [
      { label: "Productos", url: "/products", icon: IconTags },
      { label: "Tiendas", url: "/stores", icon: IconBuildingStore },
      { label: "Proveedores", url: "/suppliers", icon: IconTruck },
      { label: "Acuerdos", url: "/supply-agreements", icon: IconFileDescription },
    ],
  },
];
