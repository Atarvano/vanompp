# Changelog

## v1.3.0 — Sidebar + Services/Terminal + i18n (2026-08-18)

- Sidebar shell (200px ↔ 48px collapse) + Services / Projects / Pengembang router
- Services page tabbed: Apache | MySQL | Terminal; Terminal is MySQL client (SHOW DATABASES, history, DB selector)
- Rust `exec_sql` command for Terminal
- i18n foundation EN/ID + locale store
- Project page extracted; Pengembang page (About + System Info)
- Version 1.3.0 everywhere (package.json, tauri.conf.json, Cargo.toml)

## v1.2.0 — Stitch Full Light + Vano mascot (2026-05-13)

Portable dev env ramah pemula — gampang buat belajar web.

- Apache + MySQL satu klik Start All / Stop All + Logs per row
- Bikin project baru otomatis index.php + conn.php + .gitignore
- Nama database bebas custom input siswa
- List www + BIG URL klik copy / buka browser / buka folder
- Port bentrok Tetap pakai / Ganti + Buka log
- UI full light + mascot Vano gajah pink header 32px + icon 1024
- Installer clean tanpa debug trace www/__vano_health
- README ramah pemula + gambar mascot fixed + Tech Stack nama doang
- Fix v0.1.0 pill -> v1.2.0, package.json + tauri.conf.json bump

Files:
```
Vanompp_1.2.0_x64-setup.exe 149 MB
Vanompp_1.2.0_x64_en-US.msi 289 MB
```

## v1.1 — Bugfix not responded + port persist (2026-05-06)

- Fix not responded async polling + port persist Tetap pakai
- Orphan kill + Apache success marker
- Toast single-source autoMs

Files:
```
Vanompp_1.1_x64-setup.exe
Vanompp_1.1_x64_en-US.msi
```

## v0.1.0 — Initial (2026-05-04)

Portable dev env buat SMK. Anti bingung path localhost.

- Apache 8080 MySQL 3306 PHP 8.3 phpMyAdmin — Start All
- Create Folder — index.php + conn.php + .gitignore
- Create Database — checkbox + tombol Create DB
- Scan — list www + BIG URL Copy + Buka Browser

Files:
```
Vanompp_0.1.0_x64-setup.exe 149 MB
Vanompp_0.1.0_x64_en-US.msi 289 MB
```
