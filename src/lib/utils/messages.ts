export const MSG = {
  // Port
  portBusy: (port: number, proc?: string, suggest?: number) =>
    `Port ${port} dipakai sama ${proc || 'aplikasi lain'} (biasa IIS/Skype/XAMPP lama).${suggest ? ` Mau ganti ke ${suggest}?` : ` Saran: coba ${port + 1}`}`,
  apachePortConflict: (p: number, sugg: number) => `Port ${p} udah dipakai 😅 Coba ${sugg}?`,
  mysqlPortConflict: (p: number, sugg: number) => `Port ${p} bentrok sama MySQL lain 😅 Coba ${sugg}?`,

  // Service fail
  apacheFail: "Apache gagal start 😅 Cek error.log - biasanya butuh VC++ Redist 2015-2022 x64. Klik [Logs] buat intip.",
  apacheFailNoLog: "Apache gagal start 😅 Cek error.log — httpd.exe keluar tanpa log. Coba install VC++ Redist.",
  mysqlFail: "MySQL gagal start - coba [Repair] atau cek bin/mysql/data/mysql_error.log",
  mysqlFailNoLog: "MySQL gagal start 😅 Cek mysql_error.log — mysqld keluar tanpa log",
  vcRedistTip: "Butuh VC++ Redist 2015-2022 x64 — install dari Microsoft atau bin/apache/vc_redist.x64.exe",
  genericFail: (svc: string) => `${svc} gagal start 😅 Cek log buat detail`,

  // Project
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

  // Logs
  logEmpty: "Log belum ada... service mungkin belum pernah start",
  logNotFound: (svc: string) => `File log ${svc} belum ketemu — pastikan bin/${svc} sudah ter-install`,
  logReadFail: (e: string) => `Gagal baca log: ${e}`,

  // Generic
  openFolderFail: (e: string) => `Gagal buka folder: ${e}`,
  unexpectedError: "Waduh error tak terduga 😅 Coba restart Vanompp",

  // Hints
  installVcRedistAction: "Install VC++ Redist",
  openLogAction: "Buka Logs",
  copyUrlSuccess: "URL dicopy! Paste di browser ya 📋",
  openBrowserSuccess: (url: string) => `Buka ${url} di browser ↗`,
} as const

export type MsgKey = keyof typeof MSG
