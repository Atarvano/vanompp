import { writable } from 'svelte/store'
import { invoke } from '@tauri-apps/api/core'

export type Project = {
  name: string
  path: string
  url: string
  has_index: boolean
  has_conn: boolean
  has_gitignore: boolean
  db_exists: boolean
  db_name: string
}

export const projects = writable<Project[]>([])
export const selected = writable<string>('')

export async function refreshProjects(apachePort?: number): Promise<void> {
  try {
    const port = apachePort ?? 8080
    let list: Project[]
    if (port && port !== 8080) {
      list = await invoke<Project[]>('scan_projects_with_port', {
        apachePort: port,
        apache_port: port
      } as any)
    } else {
      list = await invoke<Project[]>('scan_projects')
    }
    // ensure db_name default
    const normalized = list.map((p: any) => ({
      ...p,
      db_name: p.db_name ?? '',
      db_exists: p.db_exists ?? false
    }))
    projects.set(normalized)
  } catch (e) {
    console.error('[vanompp] scan_projects failed:', e)
  }
}

export async function createDatabase(dbName: string, mysqlPort?: number): Promise<void> {
  const port = mysqlPort ?? 3306
  // snake + camel compat
  await invoke('create_database', {
    db_name: dbName,
    dbName: dbName,
    mysql_port: port,
    mysqlPort: port
  } as any)
}
