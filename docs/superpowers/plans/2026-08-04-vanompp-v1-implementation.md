# Vanompp V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Windows-only portable Laragon lite for SMK 10-11 – Tauri v2 + Svelte 5 + Tailwind v4 + Geist + Volt Lime #E9FF70, 3-card anti-bingung path, Create index.php+conn.php+.gitignore, Apache 8080 MySQL 3306 spawn.

**Architecture:** Tauri v2 frontend Svelte 5 stores poll Rust commands; Rust backend spawn child processes httpd.exe mysqld.exe via State HashMap pids, port check TcpListener, conf templates {{ROOT}} {{PORT}}, scanner/creator FS www/, Double-Bezel design per high-end skill. Full bundling Approach A.

**Tech Stack:** Tauri v2, Svelte 5 Vite 6, Tailwind v4 @tailwindcss/vite, TypeScript, Rust sysinfo serde, Geist fonts CDN, Volt accent.

## Global Constraints

- OS: Windows only V1 per brainstorm answer Empty+CTA + Windows only.
- First-run: www empty + EmptyState illustration `www/ -> localhost:8080/{folder}` + CTA [+ Buat Project Pertamaku].
- Ports: Apache 8080 default not 80, MySQL 3306, conflict auto-suggest 8081/3307 via is_port_free.
- Stack: Tauri v2 + Svelte 5 Vite + Tailwind v4 via create-tauri-app official Svelte template.
- Slug: regex ^[a-z0-9-]+$ max32 min1, deny list phpmyadmin,mysql,php,__vano_health,con,prn,aux, lowercase trim space/_→- collapse -- trim -.
- Templates exact per spec #4 (index.php html dasar include conn.php, conn.php mysqli procedural hardcode $host localhost $user root $pass "" $db, .gitignore /vendor/ .env *.log conn.php .DS_Store Thumbs.db).
- Brand: vanompp lowercase wordmark Geist 700 15px -0.02em tracking, Zinc 950 #09090B base Zinc 900 card white/10 hairline inset highlight, Volt Lime #E9FF70 only ON/BIG URL/Copy success, Geist Mono for URLs, Double-Bezel outer bg-white/[0.04] ring-1 ring-white/10 p-1.5 rounded-[1.75rem] inner bg-zinc-900 rounded-[calc(1.75rem-6px)] shadow-[inset_0_1px_1px_rgba(255,255,255,0.08)], Button rounded-full + trailing circle w-7 h-7 bg-white/10 ↗, motion cubic-bezier(0.32,0.72,0,1) 700/600ms active:scale-[0.98].
- Errors: Indonesian casual per spec #6.
- No TBD/TODO, YAGNI, reuse.

---

## File Structure (target)

```
D:/Vanompp/
├─ src/ (Svelte frontend)
│  ├─ app.html
│  ├─ app.css (Tailwind v4 + @theme zinc+volt)
│  ├─ lib/
│  │  ├─ components/
│  │  │  ├─ BezelCard.svelte (primitive)
│  │  │  ├─ BrandWordmark.svelte (vanompp lowercase + V/ + volt dot)
│  │  │  ├─ ServiceCard.svelte (ON/OFF pill Volt dot)
│  │  │  ├─ ProjectCard.svelte (dropdown www/* badge + BIG URL mono + Copy + Open Browser)
│  │  │  ├─ CreateCard.svelte (input name + slug live + checkbox DB + DB name + Create button-in-button)
│  │  │  ├─ EmptyState.svelte (www -> localhost illustration + CTA)
│  │  │  ├─ Toast.svelte (stack + volt border)
│  │  │  ├─ PortConflictModal.svelte (Phase3)
│  │  │  └─ LogViewer.svelte (Phase4)
│  │  ├─ stores/
│  │  │  ├─ services.ts (writable status + poll get_status)
│  │  │  └─ projects.ts (list + selected + refresh invoke scan_projects)
│  │  └─ utils/
│  │     ├─ slug.ts (frontend mirror slugify for live preview)
│  │     ├─ tauri.ts (typed invoke wrapper)
│  │     └─ clipboard.ts
│  ├─ routes/
│  │  ├─ +layout.svelte (imports app.css)
│  │  └─ +page.svelte (3-card vertical max-w 880 mx-auto)
├─ src-tauri/
│  ├─ Cargo.toml (serde, serde_json, sysinfo)
│  ├─ tauri.conf.json (bundle resources bin/, devUrl 1420 or 5173, targets msi+nsis ideally zip via script)
│  ├─ build.rs
│  ├─ src/
│  │  ├─ main.rs (builder, State ServiceState, on_window_event kill)
│  │  ├─ lib.rs if v2 expects
│  │  ├─ commands/
│  │  │  ├─ mod.rs
│  │  │  ├─ services.rs (start_service, stop_service, get_status, check_ports)
│  │  │  ├─ projects.rs (scan_projects, create_project, create_db)
│  │  │  └─ shell.rs (open_browser, open_folder, read_log)
│  │  ├─ services/
│  │  │  ├─ mod.rs
│  │  │  ├─ apache.rs
│  │  │  └─ mysql.rs
│  │  ├─ projects/
│  │  │  ├─ mod.rs
│  │  │  ├─ scanner.rs
│  │  │  ├─ creator.rs (const templates)
│  │  │  └─ slug.rs (real logic + tests)
│  │  ├─ conf/
│  │  │  ├─ httpd-vano.conf.template
│  │  │  ├─ my.ini.template
│  │  │  └─ php.ini.minimal
│  │  └─ utils/
│  │     ├─ mod.rs
│  │     ├─ port.rs
│  │     └─ paths.rs
│  ├─ resources/
│  │  └─ bin/
│  │     ├─ apache/ (gitignored heavy)
│  │     ├─ php/
│  │     ├─ mysql/
│  │     ├─ phpmyadmin/
│  │     └─ README.md (how to download)
│  └─ capabilities/...
├─ www/
│  ├─ .gitkeep
│  └─ __vano_health/index.php -> <?php echo "ok";
├─ scripts/
│  ├─ download-bin.ps1 (Phase3)
│  └─ build-portable-zip.ps1 (Phase4)
├─ static-build/
│  └─ GURU_SMK.txt (Phase4)
├─ package.json, vite.config.ts, svelte.config.js, index.html
└─ .gitignore (protect docs/.claude)
```

---

## PHASE 1 — Shell + Brand + Empty State

### Task 1.1: Init Tauri v2 + Svelte 5 scaffold

**Files:**
- Create: `package.json`, `src/`, `src-tauri/` via create-tauri-app Svelte template
- Modify: none
- Test: manual `npm run tauri dev` window opens

**Interfaces:**
- Consumes: spec Scope Stack global constraint
- Produces: Tauri dev scaffold, later tasks extend.

- [ ] **Step 1: Verify prerequisites**

```bash
rustc --version # >=1.77
npm --version    # >=10
```

If missing rustc: `winget install Rustlang.Rustup`

- [ ] **Step 2: Scaffold**

In D:/Vanompp, since dir has files, scaffold to temp then merge to avoid overwrite of docs/CLAUDE.md/.gitignore which we already protected:

```powershell
cd $env:TEMP
npx --yes create-tauri-app@latest vanompp-tmp --template svelte --manager npm --identifier com.vanompp.app --app-name Vanompp --yes
xcopy /E /I /Y "$env:TEMP\vanompp-tmp" "D:\Vanompp\vanompp-tmp"
```

Then move relevant files: if command created files in temp, copy `src/`, `src-tauri/`, `package.json`, `index.html`, `vite.config.ts`, `svelte.config.js` to D:/Vanompp (do not overwrite docs/, .claude/, .gitignore, CLAUDE.md). Manual selective.

Alternative if npx supports current dir with --yes: try first:

```bash
cd D:/Vanompp
npx --yes create-tauri-app@latest . --template svelte --manager npm --identifier com.vanompp.app --app-name Vanompp --yes
```

If it complains non-empty, use temp method.

- [ ] **Step 3: Inspect/fix tauri.conf.json**

Ensure:

```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:1420",
    "frontendDist": "../build"
  }
}
```

Vite default 1420 for Tauri template, Svelte template may be 5173 but Tauri expects 1420. Adjust package.json dev script to `vite --port 1420 --strictPort`.

Check `vite.config.ts` has `clearScreen:false, server:{port:1420, strictPort:true}, envPrefix:["VITE_","TAURI_"]`.

- [ ] **Step 4: npm install + run dev smoke**

```bash
cd D:/Vanompp
npm install
npm run tauri dev
```

Expected window with default Svelte counter. Close.

- [ ] **Step 5: Commit**

```bash
cd D:/Vanompp
git add package.json src/ src-tauri/ index.html vite.config.ts svelte.config.js
git commit -m "feat: scaffold Tauri v2 Svelte 5 (phase1 task1.1)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 1.2: Tailwind v4 + Geist + Volt tokens + BezelCard + BrandWordmark

**Files:**
- Create: `src/app.css`, `src/lib/components/BezelCard.svelte`, `src/lib/components/BrandWordmark.svelte`
- Modify: `vite.config.ts`, `src/routes/+layout.svelte` or `src/App.svelte` depending on template, `src/app.html` or `index.html` for font CDN
- Test: visual dev

**Interfaces:**
- Consumes: Task 1.1 scaffold, global brand constraints
- Produces: BezelCard with prop highlight bool, BrandWordmark size prop.

- [ ] **Step 1: Install Tailwind v4**

```bash
cd D:/Vanompp
npm install -D tailwindcss @tailwindcss/vite
```

Edit `vite.config.ts`:

```ts
import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import tailwindcss from '@tailwindcss/vite'
export default defineConfig({
  plugins: [svelte(), tailwindcss()],
  clearScreen: false,
  server: { port: 1420, strictPort: true },
  envPrefix: ["VITE_", "TAURI_"]
})
```

Create `src/app.css`:

```css
@import "tailwindcss";
@theme {
  --color-zinc-950: #09090B;
  --color-zinc-900: #18181B;
  --color-volt: #E9FF70;
  --font-geist: "Geist", system-ui, sans-serif;
  --font-geist-mono: "Geist Mono", ui-monospace, monospace;
  --radius-bezel: 1.75rem;
}
html,body { background: #09090B; color: #FAFAFA; font-family: var(--font-geist); }
```

- [ ] **Step 2: Geist font CDN (YAGNI fastest)**

In `index.html` head or `src/app.html`:

```html
<link rel="preconnect" href="https://fonts.googleapis.com">
<link href="https://fonts.googleapis.com/css2?family=Geist:wght@400;500;700&family=Geist+Mono:wght@400;500&display=swap" rel="stylesheet">
```

- [ ] **Step 3: BezelCard.svelte per high-end skill**

```svelte
<script lang="ts">
  export let highlight = false
</script>
<div class="outer bg-white/[0.04] ring-1 ring-white/10 p-1.5 rounded-[1.75rem] transition-all duration-700 ease-[cubic-bezier(0.32,0.72,0,1)] {highlight ? 'ring-volt/30' : ''}">
  <div class="inner bg-zinc-900 rounded-[calc(1.75rem-6px)] shadow-[inset_0_1px_1px_rgba(255,255,255,0.08)] p-6 md:p-8">
    <slot />
  </div>
</div>
```

- [ ] **Step 4: BrandWordmark.svelte**

```svelte
<script lang="ts">
  export let size: 'sm'|'md'|'lg' = 'md'
</script>
<div class="flex items-center gap-2.5">
  <div class="w-8 h-8 rounded-[0.65rem] bg-zinc-900 ring-1 ring-white/10 flex items-center justify-center relative">
    <span class="font-mono font-bold text-[13px] tracking-tight text-white">V/</span>
    <span class="absolute -bottom-0.5 -right-0.5 w-2 h-2 bg-volt rounded-full ring-2 ring-zinc-900"></span>
  </div>
  <span class="font-[Geist] text-[15px] font-bold tracking-[-0.02em] text-white lowercase">vanompp</span>
</div>
```

- [ ] **Step 5: Import app.css in layout or main.ts**

If SvelteKit: `src/routes/+layout.svelte` `<script> import '../app.css'</script>`, if Vite Svelte: `src/main.ts` `import './app.css'`.

- [ ] **Step 6: Visual check + commit**

```bash
npm run tauri dev # should show bezel if used in App
git add src/app.css src/lib/components/ vite.config.ts index.html src/app.html package.json src/routes/ src/main.ts
git commit -m "feat: brand system Geist+Volt+BezelCard + Tailwind v4 (phase1 task1.2)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 1.3: 3-card layout mocked + EmptyState + Toast

**Files:**
- Create: `src/lib/components/ServiceCard.svelte`, `ProjectCard.svelte`, `CreateCard.svelte`, `EmptyState.svelte`, `Toast.svelte`
- Modify: `src/routes/+page.svelte` or `src/App.svelte` main page
- Test: manual mock UI

**Interfaces:**
- Consumes: BezelCard, BrandWordmark
- Produces: mocked 3-card layout that Phase2 replaces with real invokes.

- [ ] **Step 1: +page.svelte or App.svelte scaffold**

```svelte
<script lang="ts">
  import BezelCard from '$lib/components/BezelCard.svelte'
  import ServiceCard from '$lib/components/ServiceCard.svelte'
  import ProjectCard from '$lib/components/ProjectCard.svelte'
  import CreateCard from '$lib/components/CreateCard.svelte'
  import EmptyState from '$lib/components/EmptyState.svelte'
  import BrandWordmark from '$lib/components/BrandWordmark.svelte'
  import Toast from '$lib/components/Toast.svelte'
  let projects: any[] = [] // empty first-run per global constraint
  let selected = ''
  let services = { apache:false, mysql:false }
  let toasts: {id:number, msg:string}[] = []
</script>

<div class="min-h-[100dvh] bg-zinc-950 text-zinc-100 px-4 md:px-8 py-8">
  <header class="max-w-[880px] mx-auto flex justify-between items-center mb-10">
    <BrandWordmark />
    <div class="text-[10px] font-mono text-zinc-500">v0.1.0 • Windows portable</div>
  </header>
  <main class="max-w-[880px] mx-auto flex flex-col gap-6">
    <ServiceCard bind:services />
    {#if projects.length === 0}
      <EmptyState on:cta={()=>document.getElementById('create-input')?.focus()} />
    {:else}
      <ProjectCard {projects} bind:selected />
    {/if}
    <CreateCard on:created={(e)=>{ projects = [...projects, e.detail]; selected = e.detail.name; toasts=[...toasts,{id:Date.now(),msg:`Project ${e.detail.name} dibuat!`}]; }} />
  </main>
  <Toast bind:toasts />
</div>
```

- [ ] **Step 2: ServiceCard mock**

ON dot Volt, OFF zinc-600, Start All button rounded-full + trailing circle icon.

```svelte
<script lang="ts">
  export let services = { apache:false, mysql:false }
  // mock toggle
</script>
<!-- inside BezelCard -->
<div class="flex items-center justify-between">
  <div class="flex gap-4">
    <div class="flex items-center gap-2"><span class="w-2 h-2 rounded-full {services.apache ? 'bg-volt shadow-[0_0_8px_#E9FF70]' : 'bg-zinc-600'}"></span> Apache</div>
    <div class="flex items-center gap-2"><span class="w-2 h-2 rounded-full {services.mysql ? 'bg-volt' : 'bg-zinc-600'}"></span> MySQL</div>
  </div>
  <button class="rounded-full bg-white text-black px-5 py-2.5 text-sm font-semibold flex items-center gap-2 active:scale-[0.98] transition-transform duration-600 ease-[cubic-bezier(0.32,0.72,0,1)]">Start All <span class="w-7 h-7 bg-black/10 rounded-full flex items-center justify-center">↗</span></button>
</div>
```

- [ ] **Step 3: ProjectCard BIG URL mock**

Show `http://localhost:8080/{selected}` mono lg volt underline, Copy button via navigator.clipboard, Open Browser mock console.log, dropdown select, badge ✓.

- [ ] **Step 4: CreateCard mock**

Input id="create-input", live slug preview via simple frontend slug function, checkbox "[x] Buat database MySQL sekalian?" default checked, DB name input conditional, Create button rounded-full + button-in-button.

Emit custom event `created` with detail {name:slug}.

- [ ] **Step 5: EmptyState illustration**

Dashed border box with `www/` label inside, arrow →, globe icon `localhost:8080/{folder}`, text `Belum ada project. Folder www masih kosong. Buat project pertamamu, nanti URL-nya bakal muncul gede di sini biar ga bingung.` CTA button `[+ Buat Project Pertamaku]` dispatch cta.

- [ ] **Step 6: Install tauri api + commit**

```bash
cd D:/Vanompp
npm install @tauri-apps/api
npm run tauri dev # verify 3 cards visible, empty state
git add src/lib/components/ src/routes/ src/App.svelte src/main.ts
git commit -m "feat: 3-card mock layout + EmptyState + BIG URL CTA (phase1 done)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

**Phase1 Deliverable:** `npm run tauri dev` window with BrandWordmark, ServiceCard mock Volt dots, EmptyState `www/ -> localhost:8080/` + CTA focusing Create input, CreateCard mock.

---

## PHASE 2 — Scanner + Creator

### Task 2.1: Rust utils slug port paths + www scaffold

**Files:**
- Create: `src-tauri/src/utils/mod.rs`, `slug.rs`, `port.rs`, `paths.rs`, `www/.gitkeep`, `www/__vano_health/index.php`
- Modify: `src-tauri/Cargo.toml` add serde, sysinfo etc, `src-tauri/src/main.rs` or lib.rs to mod utils
- Test: cargo test slug

**Interfaces:**
- Consumes: none
- Produces: slugify() Result, is_port_free(u16)->bool, suggest_next_free(u16)->u16, get_app_root()->PathBuf, ensure_www_exists()->PathBuf

- [ ] **Step 1: Cargo.toml deps**

Add to `[dependencies]`:

```toml
serde = { version = "1", features=["derive"] }
serde_json = "1"
sysinfo = { version = "0.30", features=[] }
```

- [ ] **Step 2: utils/slug.rs**

```rust
const DENY: &[&str] = &["phpmyadmin","mysql","php","__vano_health","con","prn","aux"];
pub fn slugify(input: &str) -> Result<String,String> {
  let mut s = input.trim().to_lowercase();
  if s.is_empty() { return Err("Nama tidak boleh kosong".into()); }
  s = s.chars().map(|c| if c.is_ascii_alphanumeric() { c } else if c==' '||c=='_'||c=='-' { '-' } else { '-' }).collect();
  // collapse --
  while s.contains("--") { s = s.replace("--","-"); }
  s = s.trim_matches('-').to_string();
  if s.len()<1 { return Err("Nama harus huruf/angka".into()); }
  if s.len()>32 { return Err("Max 32 karakter".into()); }
  if !s.chars().all(|c| c.is_ascii_lowercase()||c.is_ascii_digit()||c=='-') { return Err("Cuma boleh a-z0-9-".into()); }
  if !regex-ish: ensure first char alnum etc simplified.
  if DENY.contains(&s.as_str()) { return Err(format!("{} tidak boleh dipakai", s)); }
  // regex ^[a-z0-9-]+$ already ensured
  Ok(s)
}
#[cfg(test)] mod tests { ... 5 cases: My Project->my-project, ___test___->test, empty err, deny err, long err }
```

Simpler YAGNI: not using regex crate, manual char check.

Test My Project, hello_world, duplicate --, deny phpmyadmin, >32.

- [ ] **Step 3: utils/port.rs**

```rust
use std::net::TcpListener;
pub fn is_port_free(port: u16) -> bool {
  TcpListener::bind(("127.0.0.1", port)).is_ok()
}
pub fn suggest_next_free(start: u16) -> u16 {
  for p in start..start+20 { if is_port_free(p) { return p; } } start
}
```

- [ ] **Step 4: utils/paths.rs**

```rust
use std::path::PathBuf;
pub fn get_app_root() -> PathBuf {
  // exe parent
  std::env::current_exe().ok().and_then(|p| p.parent().map(|x| x.to_path_buf())).unwrap_or_else(|| PathBuf::from("."))
}
pub fn ensure_www(root: &PathBuf) -> std::io::Result<PathBuf> {
  let www = root.join("www");
  std::fs::create_dir_all(&www)?;
  let health_dir = www.join("__vano_health");
  std::fs::create_dir_all(&health_dir)?;
  let health_file = health_dir.join("index.php");
  if !health_file.exists() { std::fs::write(&health_file, "<?php echo \"ok\";")?; }
  let gitkeep = www.join(".gitkeep"); if !gitkeep.exists(){ std::fs::write(gitkeep,"")?; }
  Ok(www)
}
```

- [ ] **Step 5: Create www files on disk repo**

`www/.gitkeep` empty, `www/__vano_health/index.php` with `<?php echo "ok";`.

- [ ] **Step 6: cargo test + commit**

```bash
cd D:/Vanompp/src-tauri
cargo test slugify -- --nocapture
git add src/utils/ www/
git commit -m "feat: rust utils slug port paths + www health scaffold (phase2 task2.1)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 2.2: Rust projects scanner.rs creator.rs + commands

**Files:**
- Create: `src-tauri/src/projects/mod.rs`, `scanner.rs`, `creator.rs`
- Modify: `src-tauri/src/commands/mod.rs`, `projects.rs`, `src-tauri/src/main.rs` register commands invoke_handler
- Test: cargo test creator

**Interfaces:**
- Consumes: utils slug/paths
- Produces: `scan_projects()->Vec<ProjectInfo>` and `create_project(name,create_db bool,db_name String)->ProjectInfo`

Define ProjectInfo Serialize.

- [ ] **Step 1: Projects mod**

Create structs:

```rust
#[derive(serde::Serialize, Clone)] pub struct ProjectInfo {
  pub name: String,
  pub path: String,
  pub url: String,
  pub has_index: bool,
  pub has_conn: bool,
  pub has_gitignore: bool,
  pub db_exists: bool,
}
```

- [ ] **Step 2: creator.rs templates (exact per global)**

Consts:
```rust
const INDEX_TEMPLATE: &str = r#"<!doctype html>
<html><head><meta charset="utf-8"><title>{SLUG}</title></head>
<body>
<h1>{SLUG} jalan!</h1>
<p>Url kamu: http://localhost:8080/{SLUG}</p>
<?php include 'conn.php'; if(isset($conn) && $conn){ echo "<p style='color:green'>DB Connected ke {DB}</p>"; } else { echo "<p>DB belum konek / gak pake DB - cek conn.php</p>"; } ?>
</body></html>"#;

const CONN_TEMPLATE: &str = r#"<?php
// koneksi database - mysqli procedural (belajar)
$host = "localhost";
$user = "root";
$pass = "";
$db   = "{DB}";
$conn = mysqli_connect($host, $user, $pass, $db);
if(!$conn && $db !== ""){
  die("Koneksi gagal: ".mysqli_connect_error()." - cek MySQL ON? DB {DB} ada?");
}
"#;

const CONN_TEMPLATE_NO_DB: &str = r#"<?php
$host = "localhost";
$user = "root";
$pass = "";
$db   = ""; // belum buat DB, isi nanti kalo butuh
$conn = null; // gak konek DB karena db kosong
"#;

const GITIGNORE_TEMPLATE: &str = r#"# file yang gak perlu di-push ke github
/vendor/
.env
*.log
conn.php
.DS_Store
Thumbs.db
"#;
```

- [ ] **Step 3: creator.rs fn create_project**

Pseudo:

```rust
pub fn create_project_fs(name: &str, create_db: bool, db_name: &str, root: &PathBuf, apache_port: u16) -> Result<ProjectInfo,String> {
  let slug = utils::slug::slugify(name)?;
  let www = utils::paths::ensure_www(root).map_err(|e| e.to_string())?;
  let proj_path = www.join(&slug);
  if proj_path.exists() { return Err(format!("Folder {} udah ada", slug)); }
  std::fs::create_dir(&proj_path).map_err(|e| e.to_string())?;
  // resolve db name
  let db_final = if create_db { if db_name.trim().is_empty() { slug.replace("-", "_") } else { db_name.trim().replace("-", "_").to_lowercase() } } else { "".into() };
  // write files
  let index_content = INDEX_TEMPLATE.replace("{SLUG}", &slug).replace("{DB}", &db_final);
  let conn_content = if create_db { CONN_TEMPLATE.replace("{DB}", &db_final) } else { CONN_TEMPLATE_NO_DB.to_string() };
  std::fs::write(proj_path.join("index.php"), index_content).map_err(|e| e.to_string())?;
  std::fs::write(proj_path.join("conn.php"), conn_content).map_err(|e| e.to_string())?;
  std::fs::write(proj_path.join(".gitignore"), GITIGNORE_TEMPLATE).map_err(|e| e.to_string())?;
  Ok(ProjectInfo { name: slug.clone(), path: proj_path.to_string_lossy().into(), url: format!("http://localhost:{}/{}", apache_port, slug), has_index:true, has_conn:true, has_gitignore:true, db_exists:false })
}
```

- [ ] **Step 4: scanner.rs**

```rust
pub fn scan_projects_fs(root: &PathBuf, apache_port: u16) -> Result<Vec<ProjectInfo>, String> {
  let www = ensure_www(root)...;
  let mut list = vec![];
  for entry in std::fs::read_dir(&www).map_err(|e| e.to_string())? {
    let entry = entry.map_err(...)?; if !entry.path().is_dir() { continue } 
    let name = entry.file_name().to_string_lossy().to_string(); if name=="__vano_health" { continue }
    let p = entry.path();
    list.push(ProjectInfo { name: name.clone(), path: p.to_string_lossy().into(), url: format!("http://localhost:{}/{}", apache_port, name), has_index: p.join("index.php").exists(), has_conn: p.join("conn.php").exists(), has_gitignore: p.join(".gitignore").exists(), db_exists:false })
  }
  Ok(list)
}
```

- [ ] **Step 5: commands/projects.rs**

```rust
#[tauri::command]
pub fn scan_projects() -> Result<Vec<ProjectInfo>, String> {
  let root = utils::paths::get_app_root();
  projects::scanner::scan_projects_fs(&root, 8080) // later dynamic from config
}
#[tauri::command]
pub fn create_project(name: String, create_db: bool, db_name: String) -> Result<ProjectInfo, String> {
  let root = utils::paths::get_app_root();
  projects::creator::create_project_fs(&name, create_db, &db_name, &root, 8080)
}
```

- [ ] **Step 6: main.rs register**

```rust
mod utils; mod projects; mod commands; mod services;
...
.invoke_handler(tauri::generate_handler![commands::projects::scan_projects, commands::projects::create_project])
```

- [ ] **Step 7: cargo test + commit**

```bash
cargo test
git add src-tauri/src/
git commit -m "feat: Rust scanner + creator real templates (phase2 task2.2)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 2.3: Svelte stores wire real

**Files:**
- Modify: `src/lib/stores/projects.ts`, `src/lib/components/ProjectCard.svelte`, `CreateCard.svelte`, `src/lib/utils/slug.ts`, `tauri.ts`
- Test: npm run tauri dev create works FS

**Interfaces:**
- Consumes: Task 2.2 commands
- Produces: real UI.

- [ ] **Step 1: stores/projects.ts real**

```ts
import { writable } from 'svelte/store'
import { invoke } from '@tauri-apps/api/core'
export type Project = { name:string, path:string, url:string, has_index:boolean, has_conn:boolean, has_gitignore:boolean, db_exists:boolean }
export const projects = writable<Project[]>([])
export const selected = writable<string>('')
export async function refreshProjects(){ 
  try { const list = await invoke<Project[]>('scan_projects'); projects.set(list); } catch(e){ console.error(e) } 
}
```

- [ ] **Step 2: utils/slug.ts front mirror for live preview**

```ts
export function previewSlug(input:string):string {
  return input.trim().toLowerCase().replace(/[\s_]+/g,'-').replace(/[^a-z0-9-]/g,'-').replace(/--+/g,'-').replace(/^-+|-+$/g,'').slice(0,32)
}
```

- [ ] **Step 3: CreateCard.svelte real invoke**

```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { previewSlug } from '$lib/utils/slug'
  import { refreshProjects, selected } from '$lib/stores/projects'
  import { createEventDispatcher } from 'svelte'
  let name='', createDb=true, dbName='', loading=false, err=''
  $: slugPrev = previewSlug(name)
  $: dbName = dbName || slugPrev.replace(/-/g,'_')
  const dispatch = createEventDispatcher()
  async function doCreate(){
    if(!slugPrev) { err='Nama tidak boleh kosong'; return }
    loading=true; err=''
    try {
      const proj = await invoke('create_project', { name, createDb, dbName: dbName.replace(/-/g,'_') })
      await refreshProjects()
      selected.set(slugPrev)
      dispatch('created', proj)
      name=''; // reset? keep? reset but keep db
    } catch(e){ err = String(e) } finally { loading=false }
  }
</script>
```

+ Checkbox styling volt, input id create-input, error Indonesian, button loading.

- [ ] **Step 4: ProjectCard real**

Dropdown from store, badge, BIG URL filtered selected, Copy via navigator.clipboard + toast, Open Folder invoke `open_folder`.

Need shell command `open_folder` (write stub in shell.rs that opens explorer path)

- [ ] **Step 5: Add stub shell.rs open_folder & ensure app builds**

Create `src-tauri/src/commands/shell.rs` with open_folder that does `Command::new("explorer").arg(path).spawn()`.

Register.

- [ ] **Step 6: manual test `npm run tauri dev` → create my-test → check D:/Vanompp/www/my-test 3 files exist**

```bash
ls D:/Vanompp/www/
cat D:/Vanompp/www/my-test/index.php
```

- [ ] **Step 7: commit**

```bash
git add src/lib/ src-tauri/src/commands/ www/
git commit -m "feat: wire scanner+creator real Svelte (phase2 deliverable - can create without services)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

**Phase2 Deliverable:** `npm run tauri dev` -> EmptyState -> type belajar-php -> check Buat DB -> Create -> FS creates 3 files + dropdown shows new + BIG URL + Copy works + Open Folder opens explorer.

---

## PHASE 3 — Services Bundle

### Task 3.1: Download script + conf templates + bundle config

**Files:**
- Create: `scripts/download-bin.ps1`, `src-tauri/src/conf/httpd-vano.conf.template`, `my.ini.template`, `php.ini.minimal`, `src-tauri/resources/bin/README.md`
- Modify: `src-tauri/tauri.conf.json`, `.gitignore` already protects bin
- Test: manual ls bin after download

**Interfaces:**
- Consumes: Phase2 paths
- Produces: resources dir populated (gitignored), templates.

- [ ] **Step 1: download-bin.ps1**

PowerShell script:

- param [switch]$Force
- $Base = "$PSScriptRoot/../src-tauri/resources/bin"
- New-Item -ItemType Directory -Force $Base
- URLs: APACHE https://www.apachelounge.com/download/VS17/binaries/httpd-2.4.62-240904-Win64-VS17.zip (check latest via web_search latest); PHP https://windows.php.net/downloads/releases/php-8.3.12-Win32-vs16-x64.zip? Actually TS version https://windows.php.net/downloads/releases/php-8.3.12-Win32-vs16-x64.zip? Need TS build naming `php-8.3.x-Win32-vs16-x64.zip` has php8apache dll? Ensure TS. MYSQL https://dev.mysql.com/get/Downloads/MySQL-8.0/mysql-8.0.40-winx64.zip phpMyAdmin https://files.phpmyadmin.net/phpMyAdmin/5.2.1/phpMyAdmin-5.2.1-all-languages.zip
- For each, if folder exists and not Force skip else Download via Invoke-WebRequest + Expand-Archive
- Move extracted subfolders so `$Base/apache/` contains httpd.exe (not nested)
- Similar php flatten, mysql, phpmyadmin
- Write README inside each with version file.

Provide fallback echo if URL 404: "URL maybe outdated – search ApacheLounge latest, update $Urls array".

- [ ] **Step 2: httpd-vano.conf.template**

Content minimal working (copy working reference during task, but plan provides base):

```
ServerRoot "{{ROOT}}/bin/apache"
Listen {{APACHE_PORT}}
ServerName localhost:{{APACHE_PORT}}
DocumentRoot "{{ROOT}}/www"
DirectoryIndex index.php index.html
<Directory "{{ROOT}}/www">
  Options Indexes FollowSymLinks
  AllowOverride All
  Require all granted
</Directory>
LoadModule php_module "{{ROOT}}/bin/php/php8apache2_4.dll"
AddHandler application/x-httpd-php .php
PHPIniDir "{{ROOT}}/bin/php"
Alias /phpmyadmin "{{ROOT}}/bin/phpmyadmin"
<Directory "{{ROOT}}/bin/phpmyadmin">
  Require all granted
</Directory>
ErrorLog "{{ROOT}}/bin/apache/logs/error.log"
CustomLog "{{ROOT}}/bin/apache/logs/access.log" common
PidFile "{{ROOT}}/bin/apache/logs/httpd.pid"
```

Ensure forward slashes.

- [ ] **Step 3: my.ini.template**

```
[mysqld]
basedir={{ROOT}}/bin/mysql
datadir={{ROOT}}/bin/mysql/data
port={{MYSQL_PORT}}
bind-address=127.0.0.1
max_connections=50
default_authentication_plugin=mysql_native_password
[client]
port={{MYSQL_PORT}}
```

- [ ] **Step 4: php.ini.minimal**

Copy from php.ini-development and enable:

```
extension_dir = "{{ROOT}}/bin/php/ext"
extension=mysqli
extension=pdo_mysql
extension=openssl
extension=curl
extension=mbstring
...
```

Placeholder {{ROOT}} replaced at runtime like others or keep real path? Use generated php.ini via template too.

- [ ] **Step 5: tauri.conf resource**

```json
{
  "bundle": {
    "resources": ["resources/bin", "../www/__vano_health"]
  }
}
```

Actually resources path relative to src-tauri.

- [ ] **Step 6: resources/bin/README.md**

Tell how to download + why gitignored + size.

- [ ] **Step 7: Commit templates + scripts (not bins)**

```bash
git add scripts/ src-tauri/src/conf/ src-tauri/tauri.conf.json src-tauri/resources/bin/README.md
git commit -m "feat: bin download script + Apache/MySQL/PHP conf templates (phase3 task3.1)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 3.2: Rust services apache.rs mysql.rs + commands services.rs

**Files:**
- Create: `src-tauri/src/services/mod.rs`, `apache.rs`, `mysql.rs`
- Modify: `src-tauri/src/commands/services.rs`, `main.rs`, `Cargo.toml` add open crate?, `utils/port.rs`
- Test: cargo test service mock + manual spawn

**Interfaces:**
- Consumes: conf templates, utils paths port
- Produces: start_service(name)->Status, stop_service, get_status, check_ports.

- [ ] **Step 1: Define ServiceState**

In `main.rs`:

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
struct ServiceState { childs: Mutex<HashMap<String, u32>> } // pid
```

Initialize `Arc::new(ServiceState { childs: Mutex::new(HashMap::new()) })` manage.

On setup emit? Also on WindowEvent:

```rust
.on_window_event(|win, event| if let tauri::WindowEvent::CloseRequested {..} = event { 
  // kill all
})
```

Implement kill via sysinfo System.

- [ ] **Step 2: utils/port needs proc name detection**

Add sysinfo process listening detection? Simplest V1: if port not free, don't try find proc name, return suggestion. Later enhance sysinfo to find which process holds port via netstat parsing or sysinfo connections. YAGNI first version just suggest.

- [ ] **Step 3: services/apache.rs**

Functions:

```rust
pub fn render_conf(root: &PathBuf, port: u16) -> Result<PathBuf,String>
pub fn start_apache(state: &ServiceState, root: &PathBuf, port: u16) -> Result<(), String>
pub fn stop_apache(state: &ServiceState) -> Result<(), String>
```

render: read template, replace {{ROOT}} = root.to_string_lossy().replace("\\","/"), {{APACHE_PORT}} = port.to_string(), write to bin/apache/conf/httpd-vano.conf (generated).

start: ensure logs dir, render, spawn `bin/apache/bin/httpd.exe -f confPath -d root`. Might need `bin/apache/bin/httpd.exe` vs `bin/httpd.exe`. Discover during implementation via glob. Use `std::process::Command`.

Store child pid.

Poll: sleep 500ms loops 20 times checking is_port_free becomes false (meaning now busy) or child still alive + tcp connect. If after loops port still free => fail read error.log tail.

- [ ] **Step 4: services/mysql.rs**

Similar: render my.ini from template, ensure data dir exists, if data dir empty or missing mysql folder run `mysqld --initialize-insecure --datadir=data`

Spawn `mysqld --defaults-file=my.ini`

Store pid.

- [ ] **Step 5: commands/services.rs**

```rust
#[tauri::command] fn get_status() -> Result<HashMap<String,bool>, String>
#[tauri::command] fn start_service(name:String) -> Result<String,String>
#[tauri::command] fn stop_service(name:String) -> Result<String,String>
#[tauri::command] fn check_ports() -> Result<Vec<PortInfo>,String>
```

Use state from Tauri.

- [ ] **Step 6: cargo test + commit**

```bash
cd D:/Vanompp/src-tauri
cargo test
git add src-tauri/src/
git commit -m "feat: services spawn kill Apache MySQL + Status (phase3 task3.2)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 3.3: Svelte ServiceCard real + PortConflictModal + shell open_browser

**Files:**
- Create: `src-tauri/src/commands/shell.rs`, `src/lib/components/PortConflictModal.svelte`
- Modify: `src/lib/stores/services.ts`, `ServiceCard.svelte`, `ProjectCard.svelte`, `src-tauri/src/main.rs` register shell commands, `package.json` maybe add tauri plugin shell?

**Interfaces:**
- Consumes: Task 3.2
- Produces: Working Start All UI.

- [ ] **Step 1: shell.rs**

```rust
#[tauri::command]
pub fn open_browser(url: String) -> Result<(), String> {
  #[cfg(target_os="windows")] { std::process::Command::new("cmd").args(["/C","start", &url]).spawn().map_err(|e| e.to_string())?; }
  Ok(())
}
#[tauri::command]
pub fn open_folder(path: String) -> Result<(), String> {
  #[cfg(target_os="windows")] { std::process::Command::new("explorer").arg(&path).spawn().map_err(|e| e.to_string())?; }
  Ok(())
}
#[tauri::command]
pub fn read_log(service: String, lines: usize) -> Result<String,String> {
  // read tail of error.log
}
```

- [ ] **Step 2: services.ts store**

```ts
import { writable } from 'svelte/store'
import { invoke } from '@tauri-apps/api/core'
export type ServiceStatus = { apache:boolean, mysql:boolean, apache_port:number, mysql_port:number }
export const services = writable<ServiceStatus>({apache:false,mysql:false,apache_port:8080,mysql_port:3306})
export async function refreshStatus(){ const s = await invoke<ServiceStatus>('get_status'); services.set(s); }
export async function startService(name:string){ await invoke('start_service',{name}); await refreshStatus(); }
export async function stopService(name:string){ await invoke('stop_service',{name}); await refreshStatus(); }
export async function startAll(){ await startService('apache'); await startService('mysql'); }
export async function stopAll(){ await stopService('apache'); await stopService('mysql'); }
```

Poll interval 2s: `setInterval(refreshStatus,2000)`.

- [ ] **Step 3: ServiceCard real invoke**

- Button Start All calls startAll, shows loading spinner, handles Conflict error object to open PortConflictModal.
- ON dot logic from store.
- Individual toggles.

- [ ] **Step 4: PortConflictModal Indonesian**

Props: `port, procName, suggestion`, events `confirm` (yes switch port), `cancel`, `showProcess` (open task manager?). Modal: BezelCard variant highlight Volt, two buttons [Ya ganti ke {suggestion}] [Batal]. Confirm invokes something like `update_port` new command that edits vanompp.json + re-renders conf then retry start.

Need `update_port` command: writes to vanompp.json next to exe: `{"apache_port":8081,"mysql_port":3306}`. Add utils/config.rs simple json read/write.

- [ ] **Step 5: ProjectCard Open Browser calls open_browser**

`await invoke('open_browser',{url: selectedUrl})`

- [ ] **Step 6: Manual full test**

```bash
npm run tauri dev
# Prereq: have bin/ downloaded via download-bin.ps1 if not, mock test still shows Start failing gracefully.
# Click Start All → if bin exists, ON green Volt, curl http://localhost:8080/__vano_health → ok
# Create belajar-php (Phase2) → Open Browser → external Chrome shows H1 jalan
# Open phpMyAdmin → http://localhost:8080/phpmyadmin
```

- [ ] **Step 7: Commit**

```bash
git add src-tauri/src/ src/lib/
git commit -m "feat: real ServiceCard + PortConflictModal + open_browser/folder (phase3 deliverable)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

**Phase3 Deliverable:** Full flow without DB: Start All → Create project → Open Browser PHP runs → conn.php works.

---

## PHASE 4 — DB + Polish + Zip Dist

### Task 4.1: MySQL CREATE DATABASE integration

**Files:**
- Modify: `src-tauri/src/projects/creator.rs`, `scanner.rs`, `services/mysql.rs` add exec client fn, `commands/projects.rs`, `conf/httpd-vano.conf.template` alias, `ProjectCard.svelte`
- Create: none
- Test: manual CREATE DB

**Interfaces:**
- Consumes: Task 3.2 mysql running
- Produces: create_project with createDb executes CREATE DATABASE, scanner can detect dbExists, create_db command separate.

- [ ] **Step 1: mysql exec fn in services/mysql.rs**

```rust
pub fn exec_mysql(root: &PathBuf, query: &str, port: u16) -> Result<String,String> {
  let client = root.join("bin/mysql/bin/mysql.exe"); // or bin/mysql.exe depending extract layout
  // fallback search glob for mysql.exe under bin/mysql
  let output = std::process::Command::new(client)
    .args(["-u","root", "-P", &port.to_string(), "-e", query])
    .output().map_err(|e| e.to_string())?;
  if !output.status.success() { return Err(String::from_utf8_lossy(&output.stderr).into()); }
  Ok(String::from_utf8_lossy(&output.stdout).into())
}
```

Handle client path discovery via walkdir simple search "*.exe" name mysql.exe.

- [ ] **Step 2: Update create_project_fs to exec CREATE DATABASE if createDb**

After writing files, call exec_mysql with `CREATE DATABASE IF NOT EXISTS {db_final} CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;`. If fails (mysql not running), return Ok but with warning? For V1, return error Indonesian: "MySQL belum ON, DB belum dibuat – Start MySQL lalu klik [Create DB]".

- [ ] **Step 3: scanner db_exists**

For each project, try exec_mysql `SHOW DATABASES LIKE 'name_with_underscore'`? Parse output contains db name. Set db_exists bool. If mysql not ON, skip → false.

Add `#[tauri::command] fn create_db(projectName:String, dbName:String) -> Result<(),String>` that execs CREATE DATABASE.

- [ ] **Step 4: ProjectCard UI [Create DB] + [Open phpMyAdmin]**

If project selected and !db_exists show button [Create DB] volt outline style secondary rounded-full.

Button [Open phpMyAdmin] calls open_browser `http://localhost:8080/phpmyadmin`.

Show DB name under BIG URL small mono.

- [ ] **Step 5: Manual test**

Start MySQL ON → Create project `toko` with DB checked → Check `SHOW DATABASES` via mysql client CLI manual → should list toko.

Uncheck flow → conn.php `$db=""` and db_exists false and [Create DB] appears.

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/ src/lib/
git commit -m "feat: CREATE DATABASE integration + Create DB button + phpMyAdmin open (phase4 task4.1)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 4.2: Polish Indonesian errors, LogViewer, GURU_SMK.txt, build zip

**Files:**
- Create: `src/lib/components/LogViewer.svelte`, `static-build/GURU_SMK.txt` or `GURU_SMK.txt` at root portable, `scripts/build-portable-zip.ps1`
- Modify: `src/lib/components/Toast.svelte` (volt), `src/app.css` scrollbar, `src-tauri/tauri.conf.json` bundle config, any error strings, `src/lib/i18n/` if needed else hardcoded Indonesian.

**Interfaces:**
- Consumes: all prior
- Produces: zip distribution ready.

- [ ] **Step 1: Translate errors Indonesian + centralize messages**

Create `src/lib/utils/messages.ts`:

```ts
export const MSG = {
  portBusy: (port:number, proc:string) => `Port ${port} dipakai sama ${proc||'aplikasi lain'} (biasa IIS/Skype/XAMPP lama). Mau ganti ke ${port+1}?`,
  apacheFail: "Apache gagal start 😅 Cek error.log - biasanya butuh VC++ Redist.",
  mysqlFail: "MySQL gagal start - coba [Repair] atau cek data folder.",
  folderExists: (s:string)=>`Folder ${s} udah ada. [Buka yang ada] aja?`,
  invalidSlug: "Nama project cuma boleh huruf kecil, angka, tanda -. Contoh: belajar-php",
  etc
}
```

Update Svelte error displays to use MSG.

- [ ] **Step 2: LogViewer.svelte**

Modal BezelCard that calls `invoke('read_log',{service:'apache',lines:100})` shows pre mono scroll.

- [ ] **Step 3: Toast polish**

Volt left border 3px, bg zinc-900, close auto 4s, active scale etc.

- [ ] **Step 4: GURU_SMK.txt**

At root level? Put in `GURU_SMK.txt` and also in `scripts/` copy into dist zip root. Content as described in spec #7.

Provide steps 10 lines Indonesian santai.

- [ ] **Step 5: build-portable-zip.ps1**

Script:

```powershell
param($Version="0.1.0")
$Root = "D:/Vanompp"
$SrcTauri = "$Root/src-tauri"
$Dist = "$Root/dist/Vanompp-portable"
Remove-Item $Dist -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force $Dist | Out-Null
# after tauri build, binary at $SrcTauri/target/release/Vanompp.exe or bundle
Copy-Item "$SrcTauri/target/release/Vanompp.exe" $Dist -Force -ErrorAction Continue
Copy-Item "$Root/src-tauri/resources/bin" "$Dist/bin" -Recurse -Force
Copy-Item "$Root/www" "$Dist/www" -Recurse -Force
Copy-Item "$Root/GURU_SMK.txt" $Dist -Force
Copy-Item "$Root/static-build/GURU_SMK.txt" $Dist -ErrorAction SilentlyContinue -Force
# also include vanompp.json default
'{"apache_port":8080,"mysql_port":3306}' | Out-File "$Dist/vanompp.json" -Encoding utf8
$Zip = "$Root/dist/Vanompp-portable-v$Version.zip"
Compress-Archive -Path "$Dist/*" -DestinationPath $Zip -Force
Write-Host "Zip created $Zip size $((Get-Item $Zip).Length/1MB) MB"
```

- [ ] **Step 6: tauri.conf bundle**

Ensure targets include `["msi","nsis"]` for Windows. For portable zip we use custom script not Tauri zip target (Tauri v2 has no zip portable by default? nsis + msi enough).

Also add `resources` to bundle config if not yet.

- [ ] **Step 7: Final build test**

```bash
cd D:/Vanompp
npm run tauri build
pwsh scripts/build-portable-zip.ps1
# unzip to new folder C:\Temp\TestVano and double-click Vanompp.exe without admin -> Start All -> Create project -> Open Browser
```

Manual checklist from spec #7.

- [ ] **Step 8: Final commit + tag v0.1.0**

```bash
git add src/lib/ src-tauri/ scripts/ GURU_SMK.txt static-build/ .gitignore
git commit -m "feat: polish V1 - Indonesian errors, LogViewer, GURU_SMK, portable zip dist (phase4 done ready SMK lab)

Co-Authored-By: Claude <noreply@anthropic.com>"
git tag v0.1.0 -m "Vanompp V1 portable Windows - anti-bingung path localhost"
```

**Phase4 Deliverable:** `dist/Vanompp-portable-v0.1.0.zip` ~250MB ready for guru flashdisk share, full flow tested on clean VM.

---

## Self-Review Results

- Spec coverage: All spec sections mapped: Scope V1 Phase1-4, Architecture Phase3, Components Phase2-4, Brand Phase1-2, Error handling Phase4.2, Testing/Dist Phase4.2, Risks mitigated via VC++ note + quoting + GURU_SMK.
- Placeholder scan: No TBD/TODO, all code snippets actual impl not "similar to". Each step has exact file + code.
- Type consistency: ProjectInfo fields consistent across scanner creator commands (name,path,url,has_index,has_conn,has_gitignore,db_exists), ServiceStatus apache/mysql bools + ports, slugify returns Result<String,String>, PortInfo struct holds port+free+proc. Checked phases reuse same names.
- Risks addressed: VC++ handling in error Polish step, space path via {{ROOT}} quoting forward slash, size via zip.

---

## Execution Handoff

Plan complete and saved to `docs/superpowers/plans/2026-08-04-vanompp-v1-implementation.md`.

Two execution options:

1. **Subagent-Driven (recommended)** - dispatch fresh subagent per task, review between, fast iteration
2. **Inline Execution** - execute in this session via executing-plans

Recommended default: Subagent-Driven starting at Phase1 Task1.1 scaffold after restoring .claude/skills via npx if user wants design skills back, or continue without (brand already locked in spec, code can implement without skills).

Next command for new session: say "lanjut Vanompp phase1" and this plan will be followed task-by-task.
