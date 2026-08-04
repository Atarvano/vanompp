import { writable, derived, get } from 'svelte/store'
import { invoke } from '@tauri-apps/api/core'

// ---- Types from Rust ----
export type ServiceStatus = {
  name: string
  running: boolean
  pid?: number | null
  port: number
  port_free: boolean
}

export type PortInfo = {
  port: number
  free: boolean
  suggest: number
}

export type ConflictInfo = {
  name: 'apache' | 'mysql'
  port: number
  suggest: number
}

// ---- App-facing composite state (keeps old mock compat but real data) ----
export type ServicesState = {
  apache: boolean
  mysql: boolean
  apachePort: number
  mysqlPort: number
  apachePid?: number | null
  mysqlPid?: number | null
  apachePortFree: boolean
  mysqlPortFree: boolean
}

export const services = writable<ServicesState>({
  apache: false,
  mysql: false,
  apachePort: 8080,
  mysqlPort: 3306,
  apachePid: null,
  mysqlPid: null,
  apachePortFree: true,
  mysqlPortFree: true
})

export const loading = writable<{ apache: boolean; mysql: boolean; all: boolean }>({
  apache: false,
  mysql: false,
  all: false
})

export const lastError = writable<string>('')

// derived for quick checks
export const allRunning = derived(services, ($s) => $s.apache && $s.mysql)
export const anyRunning = derived(services, ($s) => $s.apache || $s.mysql)

function mapStatuses(list: ServiceStatus[]) {
  const next: Partial<ServicesState> & { apachePort: number; mysqlPort: number } = {
    apachePort: 8080,
    mysqlPort: 3306,
    apache: false,
    mysql: false
  } as any
  let apachePid: number | null | undefined = null
  let mysqlPid: number | null | undefined = null
  let apachePortFree = true
  let mysqlPortFree = true

  for (const st of list) {
    if (st.name === 'apache') {
      next.apache = st.running
      next.apachePort = st.port
      apachePid = st.pid ?? null
      apachePortFree = st.port_free
    } else if (st.name === 'mysql' || st.name === 'mariadb') {
      next.mysql = st.running
      next.mysqlPort = st.port
      mysqlPid = st.pid ?? null
      mysqlPortFree = st.port_free
    }
  }
  return { ...next, apachePid, mysqlPid, apachePortFree, mysqlPortFree } as ServicesState
}

function indonesianify(raw: string): string {
  const s = String(raw).toLowerCase()
  if (s.includes('tidak ketemu') || s.includes('tidak ada') || s.includes('not found') || s.includes('ketemu')) return raw
  if (s.includes('permission') || s.includes('access denied') || s.includes('akses ditolak')) return 'Gagal akses file — coba run as admin 🙏'
  if (s.includes('already') || s.includes('sedang berjalan') || s.includes('already running')) return 'Service udah jalan kok 🙂'
  if (s.includes('port') && (s.includes('in use') || s.includes('dipakai') || s.includes('occupied') || s.includes('already') || s.includes('conflict') || s.includes('sudah dipakai'))) {
    return `Port kepake 😅 — coba port lain`
  }
  if (s.includes('gagal') || s.includes('failed') || s.includes('error')) return raw
  return `Gagal proses 😅: ${raw}`
}

export async function refreshStatus(): Promise<void> {
  try {
    const cur = get(services)
    const list = await invoke<ServiceStatus[]>('get_status', {
      apachePort: cur.apachePort ?? 8080,
      mysqlPort: cur.mysqlPort ?? 3306,
      apache_port: cur.apachePort ?? 8080,
      mysql_port: cur.mysqlPort ?? 3306
    } as any)
    services.set(mapStatuses(list))
    lastError.set('')
  } catch (e) {
    const msg = typeof e === 'string' ? e : String(e)
    // jangan spam error di awal — simpan silent tapi log
    console.warn('[vanompp] get_status failed:', msg)
  }
}

export async function checkPorts(): Promise<PortInfo[]> {
  try {
    const cur = get(services)
    const ap = cur.apachePort ?? 8080
    const mp = cur.mysqlPort ?? 3306
    const list = await invoke<PortInfo[]>('check_ports', {
      apachePort: ap,
      mysqlPort: mp,
      apache_port: ap,
      mysql_port: mp
    } as any)
    return list
  } catch (e) {
    console.warn('[vanompp] check_ports failed:', e)
    return []
  }
}

function svcKey(name: string): 'apache' | 'mysql' {
  const n = name.toLowerCase()
  return n.includes('mysql') || n.includes('maria') ? 'mysql' : 'apache'
}

export async function startService(name: 'apache' | 'mysql', port?: number): Promise<string> {
  const key = svcKey(name)
  loading.update((l) => ({ ...l, [key]: true }))
  lastError.set('')
  try {
    const result = await invoke<string>('start_service', {
      name,
      port: port ?? null
    } as any)
    await refreshStatus()
    return result
  } catch (e) {
    const raw = typeof e === 'string' ? e : (e as any)?.toString() || 'Gagal start service'
    const friendly = indonesianify(raw)
    lastError.set(friendly)
    throw friendly
  } finally {
    loading.update((l) => ({ ...l, [key]: false }))
  }
}

export async function stopService(name: 'apache' | 'mysql'): Promise<string> {
  const key = svcKey(name)
  loading.update((l) => ({ ...l, [key]: true }))
  lastError.set('')
  try {
    const result = await invoke<string>('stop_service', { name })
    await refreshStatus()
    return result
  } catch (e) {
    const raw = typeof e === 'string' ? e : (e as any)?.toString() || 'Gagal stop service'
    const friendly = indonesianify(raw)
    lastError.set(friendly)
    throw friendly
  } finally {
    loading.update((l) => ({ ...l, [key]: false }))
  }
}

export async function startAllServices(apachePort?: number, mysqlPort?: number): Promise<string[]> {
  loading.update((l) => ({ ...l, all: true }))
  lastError.set('')
  try {
    const cur = get(services)
    const ap = apachePort ?? cur.apachePort ?? 8080
    const mp = mysqlPort ?? cur.mysqlPort ?? 3306
    const res = await invoke<string[]>('start_all_services', {
      apachePort: ap,
      mysqlPort: mp,
      apache_port: ap,
      mysql_port: mp
    } as any)
    await refreshStatus()
    return res
  } catch (e) {
    const raw = typeof e === 'string' ? e : (e as any)?.toString() || 'Gagal start semua service'
    const friendly = indonesianify(raw)
    lastError.set(friendly)
    throw friendly
  } finally {
    loading.update((l) => ({ ...l, all: false }))
  }
}

export async function stopAllServices(): Promise<string[]> {
  loading.update((l) => ({ ...l, all: true }))
  lastError.set('')
  try {
    const res = await invoke<string[]>('stop_all_services')
    await refreshStatus()
    return res
  } catch (e) {
    const raw = typeof e === 'string' ? e : (e as any)?.toString() || 'Gagal stop semua service'
    const friendly = indonesianify(raw)
    lastError.set(friendly)
    throw friendly
  } finally {
    loading.update((l) => ({ ...l, all: false }))
  }
}

// helper: detect conflict from PortInfo[] — index-aware: 0=apache,1=mysql.
// Pure fallback when called outside Svelte context; optionally reads cur ports safely.
export function toConflicts(portInfos: PortInfo[], curPorts?: { apachePort?: number; mysqlPort?: number }): ConflictInfo[] {
  const out: ConflictInfo[] = []
  // try get store only in browser, fallback to passed curPorts
  const cur = curPorts ?? (() => { try { return get(services) as any } catch { return null } })()
  for (let i = 0; i < portInfos.length; i++) {
    const pi = portInfos[i]
    if (pi.free) continue
    let name: 'apache' | 'mysql' = i === 1 ? 'mysql' : 'apache'
    if (cur) {
      if (pi.port === cur.mysqlPort) name = 'mysql'
      else if (pi.port === cur.apachePort) name = 'apache'
    }
    // legacy exact fallback
    if (pi.port === 3306) name = 'mysql'
    out.push({ name, port: pi.port, suggest: pi.suggest })
  }
  return out
}
