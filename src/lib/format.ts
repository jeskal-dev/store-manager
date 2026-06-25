export function formatDate(
  date: Date | string | number | undefined,
  opts: Intl.DateTimeFormatOptions = {},
) {
  if (!date) return '-'

  try {
    return new Intl.DateTimeFormat('en-US', {
      month: opts.month ?? 'long',
      day: opts.day ?? 'numeric',
      year: opts.year ?? 'numeric',
      ...opts,
    }).format(new Date(date))
  } catch (_err) {
    return ''
  }
}


export const formatCurrency = (value: number | null, currency: 'MN' | 'MLC' = 'MN') => {
  if (!value) return '—';
  return new Intl.NumberFormat('es-CU', {
    style: 'currency',
    currency: currency === 'MN' ? 'CUP' : 'USD',
    minimumFractionDigits: 2,
  }).format(value);
};