import { derived, get } from 'svelte/store'
import { locale, type Locale } from '$lib/stores/locale'

type MsgFn = (...args: any[]) => string
type Dict = Record<string, string | MsgFn>

const id = {
  portBusy: (port: number, proc?: string, suggest?: number) =>
    `Port ${port} dipakai sama ${proc || 'aplikasi lain'} (biasa IIS/Skype/XAMPP lama).${suggest ? ` Mau ganti ke ${suggest}?` : ` Saran: coba ${port + 1}`}`,
  apachePortConflict: (p: number, sugg: number) => `Port ${p} udah dipakai 😅 Coba ${sugg}?`,
  mysqlPortConflict: (p: number, sugg: number) => `Port ${p} bentrok sama MySQL lain 😅 Coba ${sugg}?`,
  apacheFail: "Apache gagal start 😅 Cek error.log - biasanya butuh VC++ Redist 2015-2022 x64. Klik [Logs] buat intip.",
  apacheFailNoLog: "Apache gagal start 😅 Cek error.log — httpd.exe keluar tanpa log. Coba install VC++ Redist.",
  mysqlFail: "MySQL gagal start - coba [Repair] atau cek bin/mysql/data/mysql_error.log",
  mysqlFailNoLog: "MySQL gagal start 😅 Cek mysql_error.log — mysqld keluar tanpa log",
  vcRedistTip: "Butuh VC++ Redist 2015-2022 x64 — install dari Microsoft atau bin/apache/vc_redist.x64.exe",
  genericFail: (svc: string) => `${svc} gagal start 😅 Cek log buat detail`,
  folderExists: (s: string) => `Folder "${s}" udah ada. [Buka yang ada] aja?`,
  folderExistsSuggest: (s: string) => `Folder "${s}" udah ada. Coba nama lain atau buka yang ada di www/${s}`,
  invalidSlug: "Nama project cuma boleh huruf kecil, angka, tanda -. Contoh: belajar-php",
  slugEmpty: "Nama project ga boleh kosong",
  slugTooLong: "Max 32 karakter ya",
  slugDenied: (s: string) => `Nama "${s}" ga boleh dipakai sistem (reserved: phpmyadmin, mysql, php, dll)`,
  mysqlOff: "MySQL belum ON, DB belum dibuat – Start MySQL dulu lalu klik [Create DB]",
  mysqlOffDb: "MySQL OFF — DB bakal dibikin nanti pas MySQL ON. Klik [Create DB] di card atas.",
  created: (name: string, port: number, db?: string) =>
    `Project "${name}" dibuat! URL: http://localhost:${port}/${name}${db ? ` + DB ${db}` : ''}`,
  dbCreated: (db: string) => `DB "${db}" berhasil dibuat 🎉`,
  dbExists: (db: string) => `DB "${db}" udah ada 👍`,
  dbFail: (msg: string) => `Gagal bikin DB: ${msg}`,
  logEmpty: "Log belum ada... service mungkin belum pernah start",
  logNotFound: (svc: string) => `File log ${svc} belum ketemu — pastikan bin/${svc} sudah ter-install`,
  logReadFail: (e: string) => `Gagal baca log: ${e}`,
  openFolderFail: (e: string) => `Gagal buka folder: ${e}`,
  unexpectedError: "Waduh error tak terduga 😅 Coba restart Vanompp",
  installVcRedistAction: "Install VC++ Redist",
  openLogAction: "Buka Logs",
  copyUrlSuccess: "URL dicopy! Paste di browser ya 📋",
  openBrowserSuccess: (url: string) => `Buka ${url} di browser ↗`,
  portKepake: (ports: number[]) => `Port kepake — cek ${ports.join(', ')}`,
  // shell / nav
  appSubtitle: "Apache + MySQL portable buat belajar web",
  navServices: "Services",
  navProjects: "Project",
  navPengembang: "Pengembang",
  collapse: "Ciutkan",
  expand: "Bentangkan",
  langLabel: "Bahasa",
}

const en = {
  portBusy: (port: number, proc?: string, suggest?: number) =>
    `Port ${port} is used by ${proc || 'another app'} (often IIS/Skype/old XAMPP).${suggest ? ` Try ${suggest}?` : ` Try ${port + 1}`}`,
  apachePortConflict: (p: number, sugg: number) => `Port ${p} is busy 😅 Try ${sugg}?`,
  mysqlPortConflict: (p: number, sugg: number) => `Port ${p} conflicts with another MySQL 😅 Try ${sugg}?`,
  apacheFail: "Apache failed to start 😅 Check error.log - usually needs VC++ Redist 2015-2022 x64. Click [Logs] to view.",
  apacheFailNoLog: "Apache failed to start 😅 Check error.log - httpd.exe exited without a log. Try installing VC++ Redist.",
  mysqlFail: "MySQL failed to start - try [Repair] or check bin/mysql/data/mysql_error.log",
  mysqlFailNoLog: "MySQL failed to start 😅 Check mysql_error.log - mysqld exited without a log",
  vcRedistTip: "Requires VC++ Redist 2015-2022 x64 - install from Microsoft or bin/apache/vc_redist.x64.exe",
  genericFail: (svc: string) => `${svc} failed to start 😅 Check logs for details`,
  folderExists: (s: string) => `Folder "${s}" already exists. Open the existing one?`,
  folderExistsSuggest: (s: string) => `Folder "${s}" already exists. Try another name or open www/${s}`,
  invalidSlug: "Project name may only use lowercase letters, numbers, and -. Example: belajar-php",
  slugEmpty: "Project name cannot be empty",
  slugTooLong: "Max 32 characters",
  slugDenied: (s: string) => `Name "${s}" is reserved (phpmyadmin, mysql, php, etc.)`,
  mysqlOff: "MySQL is OFF, DB not created yet - Start MySQL then click [Create DB]",
  mysqlOffDb: "MySQL OFF - DB will be created when MySQL is ON. Click [Create DB] on the card above.",
  created: (name: string, port: number, db?: string) =>
    `Project "${name}" created! URL: http://localhost:${port}/${name}${db ? ` + DB ${db}` : ''}`,
  dbCreated: (db: string) => `DB "${db}" created 🎉`,
  dbExists: (db: string) => `DB "${db}" already exists 👍`,
  dbFail: (msg: string) => `Failed to create DB: ${msg}`,
  logEmpty: "No logs yet... service may not have started",
  logNotFound: (svc: string) => `Log file for ${svc} not found - ensure bin/${svc} is installed`,
  logReadFail: (e: string) => `Failed to read log: ${e}`,
  openFolderFail: (e: string) => `Failed to open folder: ${e}`,
  unexpectedError: "Unexpected error 😅 Try restarting Vanompp",
  installVcRedistAction: "Install VC++ Redist",
  openLogAction: "Open Logs",
  copyUrlSuccess: "URL copied! Paste in your browser 📋",
  openBrowserSuccess: (url: string) => `Opening ${url} in browser ↗`,
  portKepake: (ports: number[]) => `Ports in use — check ${ports.join(', ')}`,
  appSubtitle: "Portable Apache + MySQL for learning web dev",
  navServices: "Services",
  navProjects: "Projects",
  navPengembang: "About",
  collapse: "Collapse",
  expand: "Expand",
  langLabel: "Language",
}

export const translations: Record<Locale, Dict> = { id: id as Dict, en: en as Dict }

// keep MSG for backward compat (defaults to id)
export const MSG = id
export type MsgKey = keyof typeof id

export function translate(l: Locale, key: string, ...args: any[]): string {
  const dict = translations[l] ?? id
  const v = dict[key] ?? id[key]
  if (typeof v === 'function') return (v as MsgFn)(...args)
  if (typeof v === 'string') return v
  return key
}

// reactive helper for Svelte: $t('slugEmpty') etc.
// use as: import { t } from '$lib/utils/messages'; $t('slugEmpty')
export const t = derived(locale, ($locale) => {
  return (key: string, ...args: any[]) => translate($locale as Locale, key, ...args)
})

// imperative helper for TS files: reads current locale synchronously
export function tc(key: string, ...args: any[]): string {
  return translate(get(locale) as Locale, key, ...args)
}
