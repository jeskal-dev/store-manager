export const PAYMENT_METHOD_LABELS: Record<string, string> = {
  cash: "Efectivo",
  credit_card: "Tarjeta de crédito",
  bank_transfer: "Transferencia bancaria",
  other: "Otro",
};

export function getPaymentMethodLabel(method: string): string {
  return PAYMENT_METHOD_LABELS[method.toLowerCase()] ?? method;
}
