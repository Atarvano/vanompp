<p align="center">
  <img src="public/mascot.png" width="160" height="160" alt="Vano mascot" />
  <br/>
  <b>Vano</b> — gajah Apache santai bawa CD lamanya, nemenin ngoding kelas 10-11
  <br/>
  <a href="#quickstart"><b>Quickstart</b></a> · <a href="#fitur">Fitur</a> · <a href="#cara-pakai">Cara Pakai</a> · <a href="#stack">Stack</a> · <a href="#dev">Dev</a> · <a href="#license">License</a>
</p>

# Vanompp v1.2 — Stitch light 100% SESUAI screenshot

> **Mascot Vano gajah pink** — Apache + MySQL portable anti-bingung SMK. UI light #fdfdf9, cards white border zinc-200 rounded-24, ON lime #E9FF70 dot black, Start black pill + arrow ↗ Stop text, hero URL cream #FEFCE8 huge mono centered.

## Quickstart

```bash
git clone https://github.com/Atarvano/vanompp && cd vanompp
npm i
npm run tauri dev   # FE + Rust scan_projects + bin/ apache/mysql
npm run build       # vite build -> dist/
```

Binaries `bin/apache/` `bin/mysql/` `bin/php/` portable included (x64 Windows). PHPMyAdmin optional alias `/phpmyadmin` via httpd template Rust.

## Fitur

- Apache + MySQL start/stop per-row + bulk Stop All / Start All bottom bar (port persist Tetap pakai / Ganti ke suggest)
- Port conflict modal per-row Tetap pakai + Ganti + Buka log, error red-50 mapping all
- Projects www/ auto scan + Select Project + hero cream URL copy + Buka Folder / Buka ↗ / Copy URL black pill
- Create DB amber-100 + phpMyAdmin ↗ + DB ✓ badge
- Create Project black pill + plus — slug validation lowercase-dash 32 chars, refreshProjects(ap) fix 8080 stale
- EmptyState www/ dashed + cream hero + black pill first CTA
- Toast white rounded-16 shadow 08 — autoMs 4000 single-source (App no double timeout)
- LogViewer white modal error tail 120 + Copy + Buka folder logs
- Icons Vano gajah: icon.png 1024 + 32 128@2x ico + Square*Logos + StoreLogo + public/mascot.png 512 + docs/images/

## Cara Pakai (SMK flow)

1. Start Apache + MySQL — kalau ON lime #E9FF70 dot black
2. Pilih project di Select → URL gede cream klik copy
3. Buka Folder / Buka browser ↗
4. Buat project baru via Nama Project Baru + Create Project
5. + Buat DB kalau has_conn && !db_exists (amber pill)
6. Kalau error: Logs atau toast Buka Logs — Apache cek VC++ Redist, MySQL cek data/

## Stack

- FE: SvelteKit + Svelte + Vite + TypeScript + Tailwind v4 @tailwindcss/vite + app.css light tokens (#fdfdf9 #E9FF70)
- BE: Tauri v2 Rust (scan_projects, create_project, open_project_folder, read_log 120 lines, port persist TOML + localStorage, orphan kill, repair mysql)
- Layout: App.svelte header mascot 32px rounded-10 ring zinc-200 v0.1.0 pill + max-w-720 centered + ServiceCard + Projects + Create + Toast + modal + LogViewer + poll 3s
- Assets: docs/images/mascot-512.png + mascot-original.png + public/mascot.png + src-tauri/icons/ Vano gajah

After install tree:
```
vanompp/
  bin/ apache/ mysql/ php/ phpmyadmin/
  src/ App.svelte lib/components/ lib/stores/ lib/utils/
  src-tauri/ tauri.conf.json + icons/ Vano gajah + src/lib.rs
  public/ mascot.png
  docs/images/ mascot-512.png
```

## Dev

```bash
npm run check   # svelte-check 0 errors
npm run dev     # vite only
npx graphify update .
cargo test --manifest-path src-tauri/Cargo.toml
```

Useful: `tauri.conf.json` icon array [32 128 128@2x icns ico] untouched, installerIcon ico.

## License

Non-commercial — buat kelas 10-11 SMK aja, jangan dijual.
