import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

/** Format a monetary string (e.g. "45230.00") into a locale currency string */
export function formatMoney(value: string): string {
  const num = Number.parseFloat(value);
  if (Number.isNaN(num)) return "$0";
  return new Intl.NumberFormat("es-MX", {
    style: "currency",
    currency: "MXN",
    minimumFractionDigits: 0,
    maximumFractionDigits: 0,
  }).format(num);
}

/** Format a number with locale separators */
export function formatNumber(value: number): string {
  return new Intl.NumberFormat("es-MX").format(value);
}
