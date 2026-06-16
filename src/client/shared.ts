import { z } from "zod";

// 2. Definición de ValueSchema (El corazón de la solución)
// ==========================================

/**
 * Schema que acepta múltiples tipos de valores, transformándolos
 * a sus representaciones reales cuando es necesario.
 */
export const ValueSchema = z.union([
  // Primitivos
  z.string(),
  z.number(),
  z.boolean(),
  z.null(),
  // Colecciones genéricas
  z.array(z.unknown()), // Para arrays
  z.record(z.string(), z.unknown()), // Para objetos planos
]);
