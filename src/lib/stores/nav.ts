import { writable } from 'svelte/store'

export type Page = 'services' | 'projects' | 'pengembang'
const STORAGE_KEY = 'vanompp_page'
const SIDEBAR_KEY = 'vanompp_sidebar_collapsed'

function getInitialPage(): Page {
  try {
    if (typeof localStorage === 'undefined') return 'services'
    const v = localStorage.getItem(STORAGE_KEY)
    if (v === 'services' || v === 'projects' || v === 'pengembang') return v
  } catch {}
  return 'services'
}

function getInitialCollapsed(): boolean {
  try {
    if (typeof localStorage === 'undefined') return false
    return localStorage.getItem(SIDEBAR_KEY) === '1'
  } catch {
    return false
  }
}

export const activePage = writable<Page>(getInitialPage())
export const sidebarCollapsed = writable<boolean>(getInitialCollapsed())

if (typeof window !== 'undefined') {
  activePage.subscribe((v) => {
    try {
      localStorage.setItem(STORAGE_KEY, v)
    } catch {}
  })
  sidebarCollapsed.subscribe((v) => {
    try {
      localStorage.setItem(SIDEBAR_KEY, v ? '1' : '0')
    } catch {}
  })
}
