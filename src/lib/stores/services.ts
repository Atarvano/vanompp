import { writable, derived, get } from 'svelte/store'
import { invoke } from '@tauri-apps/api/core'

// ---- Persisted port constants (per spec v1.1) ----
export const DEFAULT_APACHE_PORT = 8080
export const DEFAULT_MYSQL_PORT = 3306
export const STORAGE_KEY = 'vanompp_ports'
export const STORAGE_KEY_LEGACY = 'vanompp:ports'
export type PersistedFile = { apachePort?: number; mysqlPort?: number }

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

// ---- App-facing composite state ----
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
  apachePort: DEFAULT_APACHE_PORT,
  mysqlPort: DEFAULT_MYSQL_PORT,
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

export const allRunning = derived(services, ($s) => $s.apache && $s.mysql)
export const anyRunning = derived(services, ($s) => $s.apache || $s.mysql)

// ---- Persisted helpers (hybrid: toml disk truth + localStorage optimistic) ----
export function loadPersisted(): PersistedFile {
  try {
    if (typeof localStorage === 'undefined') return {}
    let raw = localStorage.getItem(STORAGE_KEY)
    if (!raw) raw = localStorage.getItem(STORAGE_KEY_LEGACY)
    if (!raw) return {}
    const parsed = JSON.parse(raw) as PersistedFile
    return parsed && typeof parsed === 'object' ? parsed : {}
  } catch {
    return {}
  }
}

export function savePersisted(p: PersistedFile) {
  try {
    if (typeof localStorage === 'undefined') return
    localStorage.setItem(STORAGE_KEY, JSON.stringify(p))
  } catch {}
}

export function isCustomPort(name: 'apache' | 'mysql', port: number): boolean {
  if (name === 'apache') return port !== DEFAULT_APACHE_PORT
  return port !== DEFAULT_MYSQL_PORT
}

export function getEffectivePorts(): { apache: number; mysql: number } {
  const ls = loadPersisted()
  return {
    apache: ls.apachePort ?? DEFAULT_APACHE_PORT,
    mysql: ls.mysqlPort ?? DEFAULT_MYSQL_PORT
  }
}

export async function loadPersistedFromRust(): Promise<PersistedFile> {
  try {
    const [apache, mysql] = await invoke<[number | null, number | null]>('get_persisted_ports')
    const res: PersistedFile = {}
    if (apache != null) res.apachePort = apache
    if (mysql != null) res.mysqlPort = mysql
    const cur = loadPersisted()
    if (res.apachePort != null) cur.apachePort = res.apachePort
    if (res.mysqlPort != null) cur.mysqlPort = res.mysqlPort
    if (res.apachePort != null || res.mysqlPort != null) savePersisted(cur)
    if (Object.keys(res).length > 0) {
      services.update((s) => ({
        ...s,
        apachePort: res.apachePort ?? s.apachePort,
        mysqlPort: res.mysqlPort ?? s.mysqlPort
      }))
    }
    return Object.keys(res).length ? cur : loadPersisted()
  } catch {
    return loadPersisted()
  }
}

export async function setPersistedPort(name: 'apache' | 'mysql', port: number): Promise<void> {
  try {
    await invoke('set_persisted_port', { name, port })
  } catch (e) {
    console.warn('[vanompp] set_persisted_port failed', e)
  }
  const cur = loadPersisted()
  if (name === 'apache') cur.apachePort = port
  else cur.mysqlPort = port
  savePersisted(cur)
  services.update((s) => ({
    ...s,
    apachePort: name === 'apache' ? port : s.apachePort,
    mysqlPort: name === 'mysql' ? port : s.mysqlPort
  }))
}

export async function resetPersistedPort(name: 'apache' | 'mysql'): Promise<void> {
  try {
    await invoke('reset_persisted_port_cmd', { name })
  } catch (e) {
    console.warn('[vanompp] reset failed', e)
  }
  const cur = loadPersisted()
  if (name === 'apache') delete cur.apachePort
  else delete cur.mysqlPort
  savePersisted(cur)
  services.update((s) => ({
    ...s,
    apachePort: name === 'apache' ? DEFAULT_APACHE_PORT : s.apachePort,
    mysqlPort: name === 'mysql' ? DEFAULT_MYSQL_PORT : s.mysqlPort
  }))
}

function mapStatuses(list: ServiceStatus[]) {
  const cur = (() => {
    try {
      return get(services) as ServicesState
    } catch {
      return null
    }
  })()
  const eff = getEffectivePorts()
  // ponytail: merge store > persisted > default to avoid 3306->3309 drift spam
  const next: Partial<ServicesState> & { apachePort: number; mysqlPort: number } = {
    apachePort: cur?.apachePort ?? eff.apache ?? DEFAULT_APACHE_PORT,
    mysqlPort: cur?.mysqlPort ?? eff.mysql ?? DEFAULT_MYSQL_PORT,
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
    const eff = getEffectivePorts()
    const cur = get(services)
    // if store still default, fill from effective once (avoids drift)
    const apEff = cur.apachePort === DEFAULT_APACHE_PORT ? eff.apache : cur.apachePort
    const mpEff = cur.mysqlPort === DEFAULT_MYSQL_PORT ? eff.mysql : cur.mysqlPort
    if (apEff !== cur.apachePort || mpEff !== cur.mysqlPort) {
      services.update((s) => ({
        ...s,
        apachePort: apEff,
        mysqlPort: mpEff
      }))
    }
    const cur2 = get(services)
    const list = await invoke<ServiceStatus[]>('get_status', {
      apachePort: cur2.apachePort ?? eff.apache ?? DEFAULT_APACHE_PORT,
      mysqlPort: cur2.mysqlPort ?? eff.mysql ?? DEFAULT_MYSQL_PORT,
      apache_port: cur2.apachePort ?? eff.apache ?? DEFAULT_APACHE_PORT,
      mysql_port: cur2.mysqlPort ?? eff.mysql ?? DEFAULT_MYSQL_PORT
    } as any)
    services.set(mapStatuses(list))
    lastError.set('')
  } catch (e) {
    const msg = typeof e === 'string' ? e : String(e)
    console.warn('[vanompp] get_status failed:', msg)
  }
}

export async function checkPorts(): Promise<PortInfo[]> {
  try {
    const cur = get(services)
    const eff = getEffectivePorts()
    const ap = cur.apachePort ?? eff.apache ?? DEFAULT_APACHE_PORT
    const mp = cur.mysqlPort ?? eff.mysql ?? DEFAULT_MYSQL_PORT
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
    const cur = get(services)
    const eff = getEffectivePorts()
    const finalPort = port ?? (key === 'apache' ? cur.apachePort ?? eff.apache : cur.mysqlPort ?? eff.mysql)
    const result = await invoke<string>('start_service', {
      name,
      port: finalPort,
      apachePort: key === 'apache' ? finalPort : cur.apachePort,
      mysqlPort: key === 'mysql' ? finalPort : cur.mysqlPort,
      apache_port: key === 'apache' ? finalPort : cur.apachePort,
      mysql_port: key === 'mysql' ? finalPort : cur.mysqlPort
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
    const eff = getEffectivePorts()
    const ap = apachePort ?? cur.apachePort ?? eff.apache ?? DEFAULT_APACHE_PORT
    const mp = mysqlPort ?? cur.mysqlPort ?? eff.mysql ?? DEFAULT_MYSQL_PORT
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

export async function repairMysql(): Promise<string> {
  loading.update((l) => ({ ...l, mysql: true }))
  lastError.set('')
  try {
    const res = await invoke<string>('repair_mysql')
    await refreshStatus()
    return res
  } catch (e) {
    const raw = typeof e === 'string' ? e : (e as any)?.toString() || 'Gagal repair MySQL'
    const friendly = indonesianify(raw)
    lastError.set(friendly)
    throw friendly
  } finally {
    loading.update((l) => ({ ...l, mysql: false }))
  }
}

// helper: detect conflict from PortInfo[] — index-aware: 0=apache,1=mysql.
export function toConflicts(portInfos: PortInfo[], curPorts?: { apachePort?: number; mysqlPort?: number }): ConflictInfo[] {
  const out: ConflictInfo[] = []
  const cur = curPorts ?? (() => { try { return get(services) as any } catch { return null } })()
  for (let i = 0; i < portInfos.length; i++) {
    const pi = portInfos[i]
    if (pi.free) continue
    let name: 'apache' | 'mysql' = i === 1 ? 'mysql' : 'apache'
    if (cur) {
      if (pi.port === cur.mysqlPort) name = 'mysql'
      else if (pi.port === cur.apachePort) name = 'apache'
    }
    if (pi.port === 3306) name = 'mysql'
    out.push({ name, port: pi.port, suggest: pi.suggest })
  }
  return out
}
