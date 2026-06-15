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
import type { NavMenuGroup } from "@/types/navigation";

export const SIDEBAR_NAV: NavMenuGroup[] = [
  {
    label: "General",
    items: [
      {
        label: "Tablero",
        url: "/",
        icon: IconLayoutDashboard,
      },
    ],
  },
  {
    label: "Inventario",
    items: [
      {
        label: "Inventario",
        icon: IconPackage,
        items: [
          { label: "Todos los items", url: "/inventory" },
          { label: "Movimientos", url: "/inventory/movements" },
          { label: "Reposición", url: "/inventory/restock" },
          { label: "Merma", url: "/inventory/shrinkage" },
        ],
      },
    ],
  },
  {
    label: "Ventas",
    items: [
      {
        label: "Ventas",
        icon: IconReceipt,
        items: [
          { label: "Todas las ventas", url: "/sales" },
          { label: "Registrar venta", url: "/sales/register" },
        ],
      },
    ],
  },
  {
    label: "Compras",
    items: [
      {
        label: "Compras",
        icon: IconShoppingCart,
        items: [
          { label: "Todas las compras", url: "/purchases" },
          { label: "Registrar compra", url: "/purchases/register" },
        ],
      },
    ],
  },
  {
    label: "Catálogo",
    items: [
      { label: "Productos", url: "/products", icon: IconTags },
      { label: "Tiendas", url: "/stores", icon: IconBuildingStore },
      { label: "Proveedores", url: "/suppliers", icon: IconTruck },
      { label: "Acuerdos", url: "/supply-agreements", icon: IconFileDescription },
    ],
  },
];
