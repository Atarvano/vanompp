import { writable } from 'svelte/store'

export type Locale = 'en' | 'id'

const STORAGE_KEY = 'vanompp_locale'

function getInitial(): Locale {
  try {
    if (typeof localStorage === 'undefined') return 'id'
    const v = localStorage.getItem(STORAGE_KEY)
    if (v === 'en' || v === 'id') return v
  } catch {}
  return 'id'
}

export const locale = writable<Locale>(getInitial())

if (typeof window !== 'undefined') {
  locale.subscribe((v) => {
    try {
      localStorage.setItem(STORAGE_KEY, v)
    } catch {}
  })
}

export function setLocale(v: Locale) {
  locale.set(v)
}
