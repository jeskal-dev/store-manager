const hasMessage = (err: unknown): err is { message: string } => {
  return 'message' in (err as Record<string, unknown>)
}

export function formatApiError(err: unknown, def?: string) {
  if (err instanceof Error) {
    try {
      const json = JSON.parse(err.message) as Error
      return json.message
    } catch (error) {
      if (typeof err.message === 'string') return err.message
    }
  }

  if (hasMessage(err)) return err.message

  if (def) return def
  return 'Ocurrió un error inesperado'
}
