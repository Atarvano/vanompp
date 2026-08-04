export function slugifyInput(input: string): string {
  let s = input.toLowerCase().trim()
  s = s.replace(/[\s_]+/g, '-')
  s = s.replace(/[^a-z0-9-]/g, '')
  s = s.replace(/--+/g, '-')
  s = s.replace(/^-+/, '').replace(/-+$/, '')
  return s.slice(0,32)
}

/**
 * Live preview version that mirrors Rust slugify but without Result.
 * For UI preview while typing: trim, lowercase, [\s_]+ -> '-', [^a-z0-9-] -> '-', --+ -> '-', ^-+|-+$ -> '', slice 0,32
 */
export function previewSlug(input: string): string {
  return input
    .trim()
    .toLowerCase()
    .replace(/[\s_]+/g, '-')
    .replace(/[^a-z0-9-]/g, '-')
    .replace(/--+/g, '-')
    .replace(/^-+|-+$/g, '')
    .slice(0, 32)
}

export function validateSlug(slug: string): string | null {
  if (!slug) return "Nama project ga boleh kosong"
  if (slug.length > 32) return "Max 32 karakter"
  if (!/^[a-z0-9-]+$/.test(slug)) return "Nama cuma boleh huruf kecil, angka, - . Max 32"
  const deny = ['phpmyadmin','mysql','php','__vano_health','con','prn','aux','nul','com1','com2','lpt1']
  if (deny.includes(slug)) return `Nama "${slug}" ga boleh dipakai sistem`
  return null
}
export const DENY_LIST = ['phpmyadmin','mysql','php','__vano_health','con','prn','aux','nul']
