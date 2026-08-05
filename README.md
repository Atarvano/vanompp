<p align="center">
  <img src="public/mascot.png" width="160" height="160" alt="Vano mascot gajah pink" />
  <br/>
  <b>Vanompp</b> — Apache + MySQL portable, yang mudah digunakan oleh pemula
  <br/>
  <a href="#quickstart"><b>Quickstart</b></a> · <a href="#fitur">Fitur</a> · <a href="#cara-pakai">Cara Pakai</a> · <a href="#stack">Stack</a> · <a href="#dev">Dev</a> · <a href="#license">License</a>
</p>

# Vanompp v1.2.0 — Stitch Full Light + Vano mascot

## Quickstart

```bash
git clone https://github.com/Atarvano/vanompp && cd vanompp
npm i
npm run tauri dev
npm run build
```

Binaries `bin/apache/` `bin/mysql/` `bin/php/` portable included (x64 Windows). phpMyAdmin optional alias `/phpmyadmin` via httpd template Rust.

## Fitur

- Apache + MySQL ON lime #E9FF70 dot black, OFF zinc-100, Start black pill ↗ Stop text, Logs white pill per row + bulk Stop All / Start All bottom bar (port persist Tetap pakai / Ganti ke suggest white/black)
- Port conflict modal white rounded-24 rows zinc-50 per-row Tetap pakai white + Ganti ke black + Buka log
- Projects www/ scan + Select white h-10 border zinc-200 + hero cream #FEFCE8 amber-100 rounded-16 mono 18-20 bold centered clickable copy + Buka Folder / Buka ↗ / Copy URL black pill + Logs
- Create DB custom name input siswa: sanitized lowercase dash->_ [a-z0-9_] max64, preview →, valid → Create DB amber-100 + phpMyAdmin ↗ + DB ✓ emerald badge
- Create Project black pill + + slug validation lowercase-dash + refreshProjects(ap) fix 8080 stale + Buat Database? checkbox + custom nama DB input w-180 rounded-full
- EmptyState www/ dashed zinc-200 bg-zinc-50 + cream hero + black pill first CTA + mascot Vano
- Toast white rounded-16 shadow 08-24 dot lime/red — autoMs 4000 single-source (App no double timeout)
- LogViewer white rounded-24 tabs black active pre zinc-50 mono 11px + Copy + Buka folder logs
- Icons Vano gajah pink: icon.png 1024 483KB + 32 128@2x ico + Square*Logos + StoreLogo + public/mascot.png 512 + docs/images/mascot-512.png + docs/images/mascot-original.png

## Cara Pakai (SMK flow)

1. Start Apache + MySQL — ON lime #E9FF70 dot black, kalau error modal per-port
2. Pilih project Select → hero URL cream gede klik copy
3. Buka Folder / Buka browser ↗ / Logs per Apache MySQL
4. Buat project baru Nama Project Baru → www/{slug}/ + custom DB name input
5. Existing project: input custom DB name + Create DB amber (siswa mau custom)
6. Kalau error: Logs → Buka error.log / mysql_error.log — Apache cek VC++ Redist, MySQL cek data/

## Stack

- FE: SvelteKit + Svelte + Vite + TS + Tailwind v4 @tailwindcss/vite + app.css light body #fdfdf9 selection #E9FF70
- BE: Tauri v2 Rust scan_projects create_project open_project_folder read_log 120 port persist TOML localStorage orphan kill repair mysql apache success marker
- Layout: App.svelte header mascot 32px rounded-10 ring zinc-200 v1.2.0 pill white border zinc-200 + max-w-720 centered + ServiceCard BezelCard white border zinc-200 rounded-24 shadow 1px + poll 3s + loadPersistedFromRust + modal + LogViewer
- Assets: public/mascot.png 512 RGBA + docs/images/mascot-512.png + mascot-original.png + src-tauri/icons/ Vano gajah all 15 sizes valid, tauri.conf icon array unchanged

After install tree:
```
vanompp/
  bin/ apache/ mysql/ php/ phpmyadmin/
  src/ App.svelte lib/components/ BezelCard ServiceCard ProjectCard CreateCard EmptyState Toast PortConflictModal LogViewer + lib/stores/ lib/utils/
  src-tauri/ tauri.conf.json v1.2.0 + icons/ Vano + src/lib.rs port persist
  public/ mascot.png Vano gajah 512
  docs/images/ mascot-512.png mascot-original.png
```

## Dev

```bash
npm run check   # 115 files 0 errors
npm run dev     # vite 1420
npm run build   # 89KB gzip 30KB
cargo test --manifest-path src-tauri/Cargo.toml  # 52 passed
```

## License

Non-commercial
