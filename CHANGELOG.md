# Changelog

## v1.1 - 2026-08-05 (bugfix Not Responded + Port Persist)
- Not Responded fix: `start_service`, `stop_service`, `start_all`, `stop_all` jadi `#[tauri::command(async)]` via `spawn_blocking` — window tidak freeze 3-10s
- Port conflict persist: `vanompp.toml` (atomic temp+rename) + localStorage `vanompp_ports` — klik Tetap pakai {suggest} = pakai + ingat forever, tidak auto-increment 3306->3309 tiap Stop All -> Start All
- Modal Tetap pakai {port} VOLT #E9FF70 fullwidth + chip custom port badge + x reset manual
- Apache false fail fix: error.log `AH00455 resuming normal operations` + `Starting worker threads` dianggap success — tidak kill orphan lagi pas parent->child handoff winnt mpm
- MySQL OFF balik ON fix: `stop_mysql/apache` brute `taskkill /F /IM mysqld.exe/httpd.exe` even if map kosong after rebuild HMR — OFF jadi OFF beneran
- Cargo 52 tests pass (config roundtrip/corrupt/reset) + svelte-check 115 files 0 errors

## v0.1.0 - 2026-08-05

- Portable Windows: Apache 8080 + MySQL 3306 + PHP 8.3 + phpMyAdmin
- Create Folder: nama -> slug -> `www/{slug}/` generate `index.php + conn.php + .gitignore`
- Create Database: checkbox pas create + tombol Create DB per project, `CREATE IF NOT EXISTS`
- Scan Projects: list `www/*` + badge + BIG URL Copy + Buka Browser
- Bundle: msi 289M + setup.exe 149M di `src-tauri/target/release/bundle/`
