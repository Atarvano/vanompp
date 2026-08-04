# Changelog — vanompp

## [0.1.0] - 2026-08-05 - V1 Rilis Awal

First release. USP: anti-bingung path localhost buat SMK kelas 10-11.
Spec: `docs/superpowers/specs/2026-08-04-vanompp-v1-design.md`

### Added
- **Portable Windows** — `Vanompp.exe + www/ + bin/ + GURU_SMK.txt`, no admin, offline, ~250MB zip
- **Services** — Apache 8080 + MySQL 3306 + PHP 8.3 TS + phpMyAdmin, Start/Stop All, port conflict auto-suggest 8081/3307, Log Viewer
- **Create Folder / Project** — input nama → auto slug (`my-project`), generate `www/{slug}/`:
  - `index.php` html dasar + include `conn.php`
  - `conn.php` mysqli procedural `root + "" + $db`
  - `.gitignore` vendor/env/log/conn.php
  - Validasi Indo: duplikat "Folder ada", invalid "huruf kecil angka - max 32"
- **Create Database** — 
  - Checkbox `[x] Buat DB sekalian?` pas create project (default ON)
  - Tombol `[Create DB]` per project kalau belum ada
  - `CREATE DATABASE IF NOT EXISTS` via `mysql.exe` + FS check `bin/mysql/data/{db}`
- **Scan Projects** — list `www/*` + badge ✓ `index.php` + BIG URL `http://localhost:8080/{folder}` + Copy + Buka Browser + Buka Folder
- **UI** — `vanompp` lowercase, Geist, Zinc 950 + Volt #E9FF70 accent, Double-Bezel card, 3-card vertical no sidebar

### Fixed
- Apache DocumentRoot + CustomLog module, MySQL tmpdir + ibdata1 pid lock + 1130 Host 127.0.0.1 grant, phpMyAdmin AllowNoPassword

### Stack
- Tauri v2 + Svelte 5 + Vite + Tailwind v4, 51 tests
