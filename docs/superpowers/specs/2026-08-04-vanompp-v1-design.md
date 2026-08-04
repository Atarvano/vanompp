# Vanompp V1 — Design Spec

**Date:** 2026-08-04  
**Status:** Approved via grilling (10/10 locked) + brainstorming 4 sections  
**Approach:** A – Full Bundling Portable  
**Recovered:** 2026-08-04 from session memory after git reset (original commit g1bb... lost, reconstructed verbatim)

## 1. Problem & USP

### Core Problems (SMK kelas 10-11)
1. Laragon/XAMPP kebanyakan fitur (Nginx, Redis, Mailpit, Multi-PHP, QuickApp, Node/Python) → bingung pemula.
2. Bingung path localhost: file taro di mana, URL `http://localhost/nama-folder` yang mana, gimana buka phpMyAdmin, gimana konek DB.

90% case kelas 11: bikin `index.php` tapi ga tau buka `localhost/nama-project` di mana, nyasar ke `localhost` doang atau `file://`.

### USP (Unique Selling Point)
**Anti-bingung path localhost** > Ringan (ringan = bonus, bukan jualan utama).

Success criteria: Anak SMK kelas 10 bisa Create project → lihat BIG URL → klik Buka di Browser → halaman jalan <60 detik tanpa tutorial eksternal / tanpa tanya guru.

## 2. Scope V1 vs Out-of-Scope

### V1 Must (Windows only per user answer)
- Tauri v2 + Svelte 5 + Vite + Tailwind v4 (official `create-tauri-app --template svelte`).
- Bundling binary portable: `src-tauri/resources/bin/{apache-2.4 Lounge/VC17, php-8.3 TS win64, mysql-8.0 zip, phpmyadmin-5}`
- Portable FS layout (like Laragon): `Vanompp.exe + www/ + bin/ + vanompp.json + vanompp.log + GURU_SMK.txt` — all beside exe, no PATH pollute, offline.
- Services:
  - Apache (mod_php) on/off toggle — no Nginx
  - MySQL on/off
  - PHP = module loaded in Apache, no separate php -S process, except PHP terminal button opens external terminal running php
  - phpMyAdmin aliased /phpmyadmin
- Project scanner: scan `www/*` folders, list metadata `{name, path, url, has_index, has_conn, has_gitignore, db_exists}`, badge ✓ if index.php exists, dropdown filter.
- Project creator:
  - Input: `name`, checkbox `[x] Buat database MySQL sekalian?` default checked, editable DB name auto = slug with _ (my-project → my_project)
  - Slugify: trim, lowercase, space/_ → -, regex ^[a-z0-9-]+$ max 32, deny list `phpmyadmin,mysql,php,__vano_health`, `filter(Boolean)` collapse --.
  - Generates 3 files in `www/{slug}/`:
    - `index.php` → html dasar only + include conn.php + echo connected — NOT phpinfo. Exact template below.
    - `conn.php` → `mysqli_connect` procedural php dasar, hardcode host=localhost user=root pass="" db=slug or "" if no DB. Edukasi die(mysqli_connect_error()).
    - `.gitignore` → /vendor/, .env, *.log, conn.php, .DS_Store, Thumbs.db with comment `# file yang gak perlu di-push`
  - If checkbox checked → exec `CREATE DATABASE IF NOT EXISTS slug` via mysql client root no pass.
  - Separate button [Create DB] per project list if not exists.
- UI: Single window, 3-card vertical, no sidebar (SMK anti-bingung):
  - Card 1 Status: Apache [● ON/OFF], MySQL [● ON/OFF], [Start All] [Stop All]
  - Card 2 Project Ku: Dropdown folder, [Open Folder], [Open phpMyAdmin], BIG URL `http://localhost:8080/slug` mono text-lg Volt underline + [Copy] + [Buka di Browser] (shell::open external browser). Empty → illustration `www/ → localhost:8080/{folder}` + CTA [+ Buat Project Pertamaku] focuses Create.
  - Card 3 Buat Baru: Input nama-project, checkbox DB, DB name input, [+ Create] rounded-full Double-Bezel + Button-in-Button ↗.
  - Footer Tools: [PHP Terminal], [Logs], [www folder]
- Port: Apache 8080 default (not 80 to avoid IIS/Skype/XAMPP conflict), MySQL 3306. Conflict detection via TcpListener bind + sysinfo process name. Modal: `Port 8080 dipakai [proc]. Mau ganti ke 8081? [Ya] [Lihat proses]`. Auto edit httpd-vano.conf Listen + vanompp.json, BIG URL updates.
- Health: `www/__vano_health/index.php` returns `ok` for readiness probe.
- First-run: Empty `www/` + EmptyState + CTA.
- Brand/UI: see section 5.

### Out of V1 (Backlog V2+)
- On-demand bin download (for installer kecil), multi PHP version switch, Nginx, Redis, Memcached, Mailpit, WSL/Linux/macOS, in-app pty terminal, full logs viewer, auto-update, telemetry, fancy MSI installer, Quick App WP/Laravel, NODE/Python/Go.

## 3. Architecture

```
Vanompp.exe (Tauri v2)
├─ Frontend Svelte 5 + Vite + Tailwind v4 + Geist + Volt #E9FF70
│  ├─ stores/services.ts (poll get_status 2s)
│  ├─ stores/projects.ts (scan list, selected)
│  ├─ components/BezelCard (Double-Bezel primitive), ServiceCard, ProjectCard BIG URL, CreateCard Button-in-Button, EmptyState, Toast, BrandWordmark vanompp lowercase
│  └─ invoke Tauri commands via @tauri-apps/api/core
├─ Rust Backend src-tauri/src/
│  ├─ main.rs (builder + States Arc<Mutex<HashMap>> pids + on_window_event CloseRequested kill childs)
│  ├─ commands/{mod, services.rs (start/stop/get_status/check_ports), projects.rs (scan/create/create_db), shell.rs (open_browser/open_folder)}
│  ├─ services/{mod, apache.rs (template render httpd-vano.conf {{ROOT}} {{PORT}} + spawn httpd.exe -f conf -d root), mysql.rs (initialize-insecure if data empty + spawn mysqld --defaults-file)}
│  ├─ projects/{mod, scanner.rs, creator.rs, slug.rs}
│  ├─ conf/{httpd-vano.conf.template, my.ini.template, php.ini.min}
│  ├─ utils/{mod, port.rs (is_port_free, suggest_next_free), paths.rs (app_root, ensure_www, health file)}
├─ Resources portable
│  ├─ bin/apache/{httpd.exe, conf/httpd-vano.conf (generated), logs/error.log, modules/}
│  ├─ bin/php/{php.exe, php.ini, ext/, php8apache2_4.dll}
│  ├─ bin/mysql/{bin/mysqld.exe, bin/mysql.exe client, data/, my.ini (generated)}
│  ├─ bin/phpmyadmin/
│  └─ www/ (user projects + __vano_health/index.php)
```

Data Flows:

Start All sequence:
1. check_ports 8080/3306 via TcpListener bind test
2. if conflict → return Err Conflict {port, proc_name, suggestion}
3. scaffold conf from templates replacing {{ROOT}} (forward slashed exe parent) {{APACHE_PORT}} {{MYSQL_PORT}} → write generated confs
4. ensure logs/, data/ dirs exist
5. spawn httpd.exe, mysqld.exe --defaults-file
6. store pid in ServiceState HashMap
7. poll health via Tcp connect localhost:8080 + GET /__vano_health until 200 or timeout 10s
8. Frontend status ON Volt dot.

Create flow:
slugify(name) → validate → mkdir www/slug → write index.php + conn.php + .gitignore from const templates → if createDb bool → exec mysql client `CREATE DATABASE IF NOT EXISTS dbName CHARACTER SET utf8mb4` → return ProjectInfo → Svelte auto select → scroll to ProjectCard → BIG URL pulse Volt animate 1s + toast success.

## 4. Components Detail

### Service Manager
- Shared State: `struct ServiceState { childs: Mutex<HashMap<String, Child|u32>> }` managed via tauri State.
- start_service(name): port check → render conf → spawn → save pid → poll → Ok.
- stop_service(name): lookup pid → sysinfo kill → remove.
- get_status: sysinfo check pid alive + port busy = true → ON.
- On exit: WindowEvent CloseRequested → stop_all → kill children, ensure mysqld shutdown? via mysqladmin or kill.
- Logs: tail via read_log(log_path, lines) command.

### Project Scanner
- `scan_projects()`: read_dir www/, filter dir, check existence of index.php, conn.php, .gitignore, build url `format!("http://localhost:{}/{}", apache_port, name)`, db_exists via mysql query `SHOW DATABASES LIKE 'name'` (optional phase4, V1 can false until phase4).
- V1 watch: simple poll interval 3s in Svelte (not notify crate YAGNI).

### Creator + Validations
- slugify: lower, trim, replace [\s_]+ -> -, remove [^a-z0-9-], collapse --, trim -, max 32, min 1, deny list case-insensitive `phpmyadmin,mysql,php,__vano_health,con,prn,aux`.
- Templates exact:
  - index.php:
    ```php
    <!doctype html>
    <html>
    <head><meta charset="utf-8"><title>{slug}</title></head>
    <body>
    <h1>{slug} jalan!</h1>
    <p>Url: http://localhost:8080/{slug}</p>
    <?php include 'conn.php'; if(isset($conn) && $conn){ echo "<p style='color:green'>DB Connected ke {db}</p>"; } else { echo "<p>DB belum konek / gak pake DB - cek conn.php</p>"; } ?>
    </body>
    </html>
    ```
  - conn.php:
    ```php
    <?php
    // koneksi database - mysqli procedural (belajar)
    $host = "localhost";
    $user = "root";
    $pass = "";
    $db   = "{db}"; // ganti kalo nama db beda, kosongin "" kalo belum buat db
    $conn = mysqli_connect($host, $user, $pass, $db);
    if(!$conn && $db !== ""){
      die("Koneksi gagal: ".mysqli_connect_error()." - cek MySQL ON? DB {db} ada?");
    }
    // $conn berhasil, pakai mysqli_query($conn, "SELECT ...")
    ```
    If no DB checkbox: `$db=""` comment.
  - .gitignore:
    ```
    # file yang gak perlu di-push ke github - biar repo rapih
    /vendor/
    .env
    *.log
    conn.php
    .DS_Store
    Thumbs.db
    ```
- create_project error cases: duplicate folder → error message "Folder ada, mau [Buka yang ada] ?", invalid slug → "Nama cuma boleh huruf kecil, angka, - . Max 32".
- create_db command: idempotent CREATE DATABASE IF NOT EXISTS, called by create_project when checked and separately via button.

### UI Components
- BezelCard: Double-Bezel per high-end skill Outer `bg-white/[0.04] ring-1 ring-white/10 p-1.5 rounded-[1.75rem]` Inner `bg-zinc-900 rounded-[calc(1.75rem-6px)] shadow-[inset_0_1px_1px_rgba(255,255,255,0.08)] p-6/8 transition-all duration-700 ease-[cubic-bezier(0.32,0.72,0,1)]`
- BrandWordmark: square 32px squircle rounded-[0.65rem] + V/ glyph + volt dot 6px absolute bottom-right + text `vanompp` lowercase Geist 700 15px tracking -0.02em
- ServiceCard: pill toggles Apache/MySQL, ON = Volt #E9FF70 dot 8px pulsating subtle, OFF = zinc-600. Button Start All `rounded-full px-6 py-3 bg-white text-black font-600` + trailing circle icon ↗ `w-7 h-7 bg-black/10 rounded-full flex center` + active:scale-[0.98]
- ProjectCard: dropdown `www/*` list + badge ✓ bg-volt text-black text 10px, BIG URL `font-mono text-lg tracking-tight` underline decoration volt/30, Copy via navigator.clipboard + toast `URL dicopy!`, Open Browser calls `open_browser` command external.
- CreateCard: input with slug preview realtime under, checkbox custom styled volt, DB name input visible only if checked, Create btn same Double-Bezel style. On success scrollIntoView ProjectCard + pulse.
- EmptyState: illustration dashed border box `www/` → arrow → globe `localhost:8080/{folder}`, text edukasi, CTA button primary.
- Toast: fixed bottom-right stack, volt left border.

## 5. Brand & Visual System (from brandkit + high-end-visual-design + design-taste-frontend)

Analyzed as: Desktop devtool for SMK 10-11, premium Linear/Raycast minimal, not landing page but devtool shell. DESIGN_VARIANCE:6 MOTION:5 DENSITY:4 (airy, not cockpit scary).

- Name: `vanompp` lowercase wordmark friendly ownable vs XAMPP uppercase.
- Logo Concept: V/ glyph — V = two slashes path / , tray like opened folder. Meaning: Vanompp = tempat project ketemu jalannya. App icon 512 single V/ in squircle 24px radius, off-black bg + volt accent dot. No server icon generic. Minimal symbolic scalable 16px tray.
- Typography: Display/UI Geist (ban Inter/Roboto per high-end), Geist Mono for URL `http://localhost:8080/...` & terminal & logs. Heading scale compact `text-[15px] font-[650] tracking-tight` — dense devtool not hero landing.
- Palette – banned AI purple:
  - Base: Zinc 950 #09090B (not pure #000)
  - Surface card: Zinc 900 #18181B
  - Border: white/10 hairline + inner highlight inset white 8%
  - Text: Zinc 100 primary, Zinc 500 secondary, Zinc 600 muted
  - Accent single: Volt Lime #E9FF70 – used ONLY for ON dot, BIG URL highlight, Copy success, pulse. No other colors. Contrast AA pass on Zinc 950.
- Double-Bezel Execution (mandatory per high-end):
  ```
  Outer: bg-white/[0.04] ring-1 ring-white/10 p-1.5 rounded-[1.75rem]
  Inner: bg-zinc-900 rounded-[calc(1.75rem-6px)] shadow-[inset_0_1px_1px_rgba(255,255,255,0.08)]
  ```
- Button-in-Button trailing icon: if button has ↗, NEVER naked, must be nested inside `w-7 h-7 rounded-full bg-white/10 flex items-center justify-center` flush right inner padding.
- Spatial: sections py-24? Not for devtool but gap-6 between cards, max-w 880px centered, breathing.
- Motion: cubic-bezier(0.32,0.72,0,1) 700ms for cards, 600ms for buttons, active scale 0.98, staggered entry delay 60ms via IntersectionObserver? Svelte transition slide. No ease-in-out/liner. GPU transform/opacity only.

## 6. Error Handling (Bahasa Indonesia santai, bukan stacktrace)

- Port busy 8080: Modal "Port 8080 dipakai sama {proc} (misal Skype/IIS/XAMPP lama). Mau ganti ke 8081? [Ya ganti ke 8081] [Lihat proses di Task Manager]"
- Port busy 3306: similar → suggest 3307.
- Apache fail start: toast "Apache gagal start 😅 Cek error.log" + show tail 20 lines + btn [Buka Logs] + [Install VC++ Redist] action `winget install Microsoft.VCRedist.2015+.x64` or prompt download exe.
- MySQL data corrupt: "MySQL data error. Coba Repair?" btn [Repair] → deletes *.pid, checks my.ini.
- www not readable: "Folder www gak bisa dibaca di {path} - cek permission / antivirus" [Buka Explorer]
- Duplicate slug: "Folder {slug} udah ada. [Buka yang ada] [Batal]"
- Invalid slug: "Nama project cuma boleh huruf kecil angka - . Contoh: belajar-php"
- MySQL root no pass fail (future secure): "Koneksi MySQL gagal – coba Start MySQL dulu?"
- All toasts Indonesian casual, not English technical.

## 7. Testing & Distribution

### Testing V1 manual-first (Tauri + bin native hard to unit fully):
- Unit Rust: slugify, port_check, template rendering, path quoting (space handling).
- e2e smoke script manual checklist:
  1. Clean Windows VM (no prior XAMPP/Laragon).
  2. Extract zip → double-click Vanompp.exe → window opens <2s.
  3. First-run empty state illustration visible + CTA focuses Create.
  4. Click Start All → Apache ON Volt dot + MySQL ON + curl http://localhost:8080/__vano_health 200 ok + http://localhost:8080/phpmyadmin 302/200.
  5. Create project `belajar-php` with DB checked → FS check www/belajar-php has 3 files → index.php content has slug → conn.php has $db=belajar_php? Actually slug db name handles _ version → DB exists `SHOW DATABASES` → BIG URL `http://localhost:8080/belajar-php` → Copy → clipboard → Buka di Browser → external browser opens with H1 jalan + DB Connected.
  6. Create with DB unchecked → conn.php $db="" → no DB created.
  7. Test port conflict: run `python -m http.server 8080` then Start All → modal suggest 8081 → Ya → BIG URL updates to :8081.
  8. Stop All → ports free.
- No Playwright V1 YAGNI, cargo test only.

### Distribution V1
- `npm run tauri build` → generates MSI + exe in src-tauri/target/release/bundle.
- Custom portable zip script `scripts/build-portable-zip.ps1`: copies Tauri exe + resources/bin + www skeleton (.gitkeep + __vano_health) + GURU_SMK.txt into `dist/Vanompp-portable/` then Compress-Archive → `Vanompp-portable-v0.1.0.zip`.
- Size estimate ~250MB zip (MySQL ~200MB majority). Acceptable for SMK flashdisk share.
- Include README `GURU_SMK.txt` content per spec:
  ```
  Vanompp V1 - Cara pakai di lab SMK:
  1. Extract zip Vanompp-portable.zip ke D:\ atau folder tanpa spasi (e.g. D:\Vanompp) biar path aman
  2. Double-click Vanompp.exe (gak perlu install)
  3. Klik [Start All] tunggu hijau ON (Apache & MySQL)
  4. Di card "Buat Baru" isi nama-project contoh: belajar-php, centang Buat DB?, klik Create
  5. Lihat BIG URL http://localhost:8080/belajar-php + klik [Buka di Browser]
  6. Folder project ada di www/belajar-php - edit pakai VS Code / Notepad++
  7. Buka phpMyAdmin via tombol di aplikasi: http://localhost:8080/phpmyadmin
  8. Koding conn.php sudah ada template mysqli_connect procedural
  9. Kalau Port busy: klik Ya ganti ke 8081
  10. Kalau Apache gagal: instal VC++ Redist (link di aplikasi) atau jalankan vc_redist.x64.exe di bin/apache/
  ```
- No auto-update, no telemetry V1.
- www/.gitkeep + __vano_health preserved.

## 8. Phased Plan (Owner request: ga one-shot)

### Phase 1 — Shell + Brand + Empty State (Day 1-2)
- `npx create-tauri-app@latest . --template svelte --manager npm --identifier com.vanompp.app --app-name Vanompp --yes` (if clash use tmp copy method)
- Tailwind v4 setup via @tailwindcss/vite plugin, Geist CDN or @fontsource, app.css with theme zinc+volt tokens, radius bezel.
- Components BezelCard, BrandWordmark primitives per high-end skill.
- 3-card scaffold mocked data no Rust invoke yet.
- EmptyState illustration SVG + CTA [+ Buat Project Pertamaku] focuses Create input.
- Deliverable: Tauri window opens with high-end UI mock, `npm run tauri dev` works.

### Phase 2 — Project Scanner + Creator (Day 2-3)
- Rust utils: slug.rs (unit tests), port.rs, paths.rs, ensure www exists + health file.
- Rust projects: scanner.rs (read_dir www), creator.rs (templates const + mkdir + write 3 files).
- Tauri commands: scan_projects, create_project via generate_handler.
- Svelte stores projects.ts + selected, tauri wrapper, slug util.
- Wire CreateCard real invoke, ProjectCard dropdown real, Copy clipboard, Open Folder via shell, EmptyState CTA.
- Deliverable: Can create project without services running, FS verification.

### Phase 3 — Portable Services Bundle (Day 3-5)
- Download script `scripts/download-bin.ps1`: Apache Lounge VC17 win64, PHP 8.3 Thread Safe win64 zip, MySQL 8.0/8.4 Community zip, phpMyAdmin zip, extract to src-tauri/resources/bin/.
- Conf templates: httpd-vano.conf.template with {{ROOT}} {{APACHE_PORT}} placeholders + LoadModule php_module + AddHandler + Alias /phpmyadmin, my.ini.template with datadir/port, php.ini minimal enabling mysqli.
- tauri.conf bundleResources ["resources/bin"].
- Rust services: apache.rs spawn/kill, mysql.rs initialize-insecure if data empty + spawn, ServiceState HashMap pids, get_status via sysinfo + Tcp.
- Shell commands: open_browser, open_folder via tauri-plugin-shell or opener crate.
- ServiceCard real wiring + PortConflictModal Indonesian.
- Health poll __vano_health.
- Deliverable: Full dev flow Start All → Create → Open Browser → PHP executes → conn.php check works (without DB).

### Phase 4 — DB Integration + Polish + Zip Distribution (Day 5-6)
- MySQL client exec fn, CREATE DATABASE integration checked box, dbExists check SHOW DATABASES LIKE, separate create_db command.
- phpMyAdmin alias ensures working, button Open phpMyAdmin.
- PortConflictModal auto-switch edits config + vanompp.json.
- Polish Indonesian errors, LogViewer component reading error.log tail, Toast styling Volt + active scale.
- Build scripts: GURU_SMK.txt, build-portable-zip.ps1.
- Build test npm run tauri build + pwsh build zip + manual QA clean VM.
- Deliverable: V1 zip ready flashdisk SMK lab, git tag v0.1.0.

## 9. Risks & Mitigations
- VC++ Redist missing on SMK PCs → bundle vcruntime dlls or prompt winget + include vc_redist.x64.exe + toast action.
- MySQL data path with space → always quote paths in conf templates, use forward slash, wrap in ".
- Antivirus false positive on mysqld/httpd spawn → note in GURU_SMK, future code signing.
- Port 80/443 blocked policy lab → use 8080 default mitigates.
- Size 250MB zip → acceptable (Laragon similar), future V2 on-demand download to reduce installer base.
- SvelteKit vs Vite template confusion → choose Vite Svelte (not SvelteKit) for simplicity no SSR.

## 10. Decisions Locked (from 10/10 grilling)
1. USP anti-bingung path localhost > ringan
2. Core Apache + 1 PHP + MySQL + phpMyAdmin + PHP Terminal. Buang Nginx/Redis/Mailpit/Multi-PHP/Node/QuickApp
3. Layout: 1 window, 3-card vertical, Dropdown scan www/* + badge ✓, BIG URL + Copy + Open Browser external
4. Create generates 3 files: index.php html dasar, conn.php mysqli procedural hardcode, .gitignore (/vendor/ .env *.log conn.php .DS_Store Thumbs.db)
5. Bundling: Option A full portable via src-tauri/resources/bin/
6. Port: 8080 default + 3306 + auto-detect conflict + popup switch 8081/3307 + phpMyAdmin included + auto CREATE DATABASE
7. DB: Checkbox [x] Buat DB? default on editable name, [Create DB] separate button, $db="" if uncheck
8. Stack: Tauri v2 + Svelte 5 + Vite + Tailwind v4 + @tauri-apps/api
9. Brand: vanompp lowercase, Geist Sans + Geist Mono, Zinc 950 #09090B base, Zinc 900 card, white/10 hairline + inset, Volt Lime #E9FF70 accent only ON/BIG URL/Copy success, Double-Bezel p-1.5 rounded-[1.75rem], Button-in-Button w-7 bg-white/10 rounded-full ↗, motion cubic-bezier(0.32,0.72,0,1)
10. Dist: Free OSS portable zip Windows only V1, MSI secondary, no auto-update/telemetry, GURU_SMK.txt for guru share via flashdisk.

---
Author: Grilling 10/10 + brainstorming 4 sections 2026-08-04, reconstructed after git reset incident
Notes: .claude/skills/ brandkit, design-taste-frontend, high-end-visual-design were deleted in reset, need npx add skill to restore, but design system already locked here so new session can continue.
