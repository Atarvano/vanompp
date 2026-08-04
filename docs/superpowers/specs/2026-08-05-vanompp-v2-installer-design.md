# Vanompp V2 — Installer + Portable Dual Distribution

**Date:** 2026-08-05
**Status:** Approved via brainstorming (user chose C dual + B Hybrid + Desktop+StartMenu + save-on-uninstall)
**Depends on:** V1 spec 2026-08-04 (Approach A Full Bundling Portable), Phase1 completed bb4bce8 + cbffaa7
**Approach:** B Hybrid — Portable folder as source of truth, NSIS wrapper outside wraps it

## 1. Problem V2

V1 portable zip works (extract anywhere, double-click exe <2s) but SMK users accustomed to Laragon/XAMPP `Next Next Finish` installer that creates `C:\laragon` folder + Desktop shortcut. Some school labs block zip extract or want standard installer for `vanompp.my.id` download page.

Need two artifacts from same exe, zero code change to portable, both safe:
- `Vanompp-portable-vX.zip` — no installer, extract anywhere (V1 existing)
- `Vanompp_X_x64-setup.exe` — installer to `C:\Vanompp` (browseable to D:), creates shortcuts, uninstaller asks keep www/data

V1 Q: "Itu semua di git history udah difixing ga ada co author kah?" → fixed, no Co-Authored-By in log.
Q: "phase 1 outputnya apa?" → Phase1 mock 3-card delivered.
Q: "serena write kan bisa ... .superpowers ini sembuyinkan juga gitignore" → fixed .gitignore: .superpowers/ + .serena/ ignored, .gitignore self-ignore removed.
Q: "cargo metadata program not found" + "link.exe not found" → env needs rustup + MSVC BuildTools (1.2GB minimal components, not 4GB full) — Phase3+4 needs it, Phase1-2 can `npm run dev`.
Q: "Wait kan ntar ada proses build to exe ... befungsi buat itu kah" → yes MSVC for `tauri build` exe.
Q: "tetap bikin folder jadi install exe gitu ... v2 aja bikin kayak gitu" + "web vanompp.my.id ada dua opsi portable atau installer" + B HYBRID selected.

Success V2: user visits vanompp.my.id, sees two buttons, either works: unzip then exe, or setup.exe Next Next Finish → C:\Vanompp with shortcuts, Start All, Create project, BIG URL, tasks safe on uninstall.

## 2. Scope V2 vs Non-Goals

**V2 Must**
- Dual distribution: portable zip + NSIS installer exe from same `dist/Vanompp-portable/` folder (B Hybrid)
- Installer defaults `C:\Vanompp`, browse page allows D:\Vanompp, E:\Tools\Vanompp etc — like Laragon, NOT `Program Files` (to keep www writable beside exe, no AppData split)
- Shortcuts: Desktop `Vanompp.lnk` + Start Menu `Vanompp\Vanompp.lnk` + `Vanompp\Uninstall.lnk` — both checked default (user answer)
- Uninstaller asks `Simpan project www/ + mysql data? Ya/Hapus` — YES keeps `www/` + `bin/mysql/data/` + `vanompp.json` + opens Explorer to show tasks safe, NO full `RMDir /r` (user answer: save prompt)
- Portable remains 0% changed, source of truth — confirms Q: "walaupun v2 installer tapi untuk portable.zip tetap aman kan ... code ga keubah"
- No change to `paths.rs` — still `exe.parent` works for both (C:\Vanompp\Vanompp.exe → C:\Vanompp\www\ and D:\Random\Vanompp-portable\Vanompp.exe → D:\Random\www\)
- `vanompp.my.id` download page offers two options (user answer C)
- Build scripts: `scripts/build-portable-zip.ps1` (V1) + new `scripts/build-installer.ps1` + `installer/installer.nsi`
- VC++ Redist handling: installer bundles `vc_redist.x64.exe` auto install /quiet if registry `HKLM\SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64` missing; portable keeps runtime toast manual "[Download VC++]" linking https://aka.ms/vs/17/release/vc_redist.x64.exe
- Size same ~250MB zip & ~250MB installer (MySQL 200MB majority)

**V2 Out-of-Scope / Non-Goals**
- NOT moving to Program Files\Vanompp + www in Documents (would require paths.rs split portable vs installed, adds complexity, SMK bingung)
- NOT changing Rust/Svelte code for installer — zero logic change, only packaging
- NOT auto-update / telemetry
- NOT MSI WIX — NSIS only (Laragon-style, simpler, smaller)
- NOT removing portable — portable stays primary for flashdisk

## 3. Architecture

```
// V2 Build Flow B Hybrid
vite build (57kB) + cargo build --release → src-tauri/target/release/vanompp.exe (8-15MB)
  │
  ├─ scripts/build-portable-zip.ps1 (V1)
  │   copies exe + src-tauri/resources/bin/{apache,php,mysql,phpmyadmin} (250MB) + www/.gitkeep + www/__vano_health/index.php + GURU_SMK.txt + vanompp.json template
  │   → dist/Vanompp-portable/
  │       ├── Vanompp.exe
  │       ├── www/.gitkeep, __vano_health/index.php (<?php echo "ok";)
  │       ├── bin/apache/{httpd.exe, conf/httpd-vano.conf.template, logs/}
  │       ├── bin/php/{php.exe, php.ini.tmpl, php8apache2_4.dll, ext/}
  │       ├── bin/mysql/{bin/mysqld.exe, data/, my.ini.tmpl}
  │       ├── bin/phpmyadmin/
  │       ├── vanompp.json (ports default 8080/3306)
  │       └── GURU_SMK.txt
  │   → Compress-Archive → dist/Vanompp-portable-v0.1.0.zip
  │
  └─ scripts/build-installer.ps1 (V2 new)
      requires dist/Vanompp-portable/ exists (ensures portable safe)
      requires makensis (NSIS) in PATH — winget install NSIS
      runs makensis installer/installer.nsi
      → dist/Vanompp_0.1.0_x64-setup.exe (NSIS, ~250MB + installer overhead ~2MB)
```

Runtime same for both:
```
Vanompp.exe launch → utils/paths.rs get_app_root() = exe.parent PathBuf
→ ensure_www(root) creates www/.gitkeep + __vano_health if missing
→ find bin/apache/bin/httpd.exe via 6 candidates relative to root
→ services apache/mysql spawn httpd.exe -f conf -d root / mysqld --defaults-file
→ stores/services.ts poll get_services_status 2s → ServiceCard Volt dot ON
→ stores/projects.ts scan_projects → www/* list → ProjectCard BIG URL localhost:{port}/{slug}
```

Why B Hybrid keeps portable safe: installer.nsi does `File /r "..\dist\Vanompp-portable\*.*"` — it never touches src/ or src-tauri/, only consumes output of portable build. Portable zip generation unchanged, so answer to "portable.zip tetap aman kan code ga keubah" = yes, zero src change.

## 4. Components & Files

**Unchanged (Portable safe proof)**
- `src-tauri/src/utils/paths.rs` — `get_app_root(): PathBuf = current_exe.parent` stays. If exe at `C:\Vanompp\Vanompp.exe`, root = `C:\Vanompp`, so `www` = `C:\Vanompp\www`, `bin` = `C:\Vanompp\bin`. If portable at `D:\Flash\Vanompp\Vanompp.exe`, root = `D:\Flash\Vanompp`. No branching.
- `src-tauri/src/utils/slug.rs`, `port.rs`, `src/lib.rs`, `src/App.svelte`, components BezelCard etc — no change
- `src-tauri/Cargo.toml`, `tauri.conf.json` bundle — V2 sets bundle targets to `[]` or keep exe only; NSIS is custom outside tauri (avoids tauri NSIS defaulting to Program Files). Alternative keeps tauri NSIS but with custom template setting InstallDir to C:\Vanompp — decision: custom outside (B) to avoid tampering tauri bundler.

**Changed / Added V2**

1. `scripts/build-portable-zip.ps1` — already planned V1 spec section 7. Implementation:
   ```powershell
   $root = Split-Path $PSScriptRoot -Parent
   $distPortable = "$root/dist/Vanompp-portable"
   Remove-Item $distPortable -Recurse -Force -ErrorAction SilentlyContinue
   New-Item -ItemType Directory -Force $distPortable | Out-Null
   Copy-Item "$root/src-tauri/target/release/vanompp.exe" "$distPortable/Vanompp.exe" -Force
   Copy-Item "$root/src-tauri/resources/bin" "$distPortable/bin" -Recurse -Force
   Copy-Item "$root/www" "$distPortable/www" -Recurse -Force # but ensure .gitkeep+health
   Copy-Item "$root/GURU_SMK.txt" "$distPortable/" -Force -ErrorAction SilentlyContinue
   # ensure www/.gitkeep exists, __vano_health/index.php == <?php echo "ok";
   Compress-Archive -Path "$distPortable/*" -DestinationPath "$root/dist/Vanompp-portable-v$Version.zip" -Force
   ```
   Note `resources/bin/` gitignored ~250MB, script handles missing with warning "Run download-binaries.ps1 first".

2. `installer/installer.nsi` — new NSIS v3 script, Laragon-style:
   ```
   !include "MUI2.nsh"
   Name "Vanompp"
   OutFile "..\dist\Vanompp_0.1.0_x64-setup.exe" ; version injected via /DVERSION
   InstallDir "C:\Vanompp"
   InstallDirRegKey HKLM "Software\Vanompp" "Install_Dir"
   RequestExecutionLevel admin ; needed for C:\ write + VC++ install
   Icon "..\src-tauri\icons\icon.ico"
   UninstallIcon same
   !define MUI_WELCOMEPAGE_TITLE "Vanompp Setup"
   !define MUI_FINISHPAGE_RUN "$INSTDIR\Vanompp.exe"
   !define MUI_FINISHPAGE_RUN_TEXT "Run Vanompp now"
   Pages: MUI_PAGE_WELCOME, MUI_PAGE_DIRECTORY (browse D:), MUI_PAGE_INSTFILES, MUI_PAGE_FINISH
   !insertmacro MUI_LANGUAGE "English"
   Section "Main" SEC_MAIN
     SetOutPath $INSTDIR
     File /r "..\dist\Vanompp-portable\*.*"
     File /nonfatal "vc_redist.x64.exe" ; bundled if exists
     WriteRegStr HKLM "Software\Vanompp" "Install_Dir" "$INSTDIR"
     WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Vanompp" "DisplayName" "Vanompp"
     WriteRegStr ... UninstallString "$INSTDIR\Uninstall.exe"
     CreateDirectory "$SMPROGRAMS\Vanompp"
     CreateShortCut "$SMPROGRAMS\Vanompp\Vanompp.lnk" "$INSTDIR\Vanompp.exe" "" "$INSTDIR\Vanompp.exe" 0
     CreateShortCut "$SMPROGRAMS\Vanompp\Uninstall.lnk" "$INSTDIR\Uninstall.exe"
     CreateShortCut "$DESKTOP\Vanompp.lnk" "$INSTDIR\Vanompp.exe" "" "$INSTDIR\Vanompp.exe" 0
     WriteUninstaller "$INSTDIR\Uninstall.exe"
     # VC++ check: ReadRegDword bExists... if not, ExecWait '"$INSTDIR\vc_redist.x64.exe" /quiet /norestart'
   SectionEnd
   Section "Uninstall"
     MessageBox MB_YESNO|MB_ICONQUESTION "Simpan project www/ & database mysql/data?\n\nYa = simpan tugas, folder www tetap ada (aman)\nTidak = hapus semua" IDYES keep IDNO nokeep
     keep:
       Delete "$INSTDIR\Vanompp.exe"
       Delete "$INSTDIR\Uninstall.exe"
       RMDir /r "$INSTDIR\bin\apache"
       RMDir /r "$INSTDIR\bin\php"
       RMDir /r "$INSTDIR\bin\phpmyadmin"
       # keep bin/mysql/data + www
       Delete "$DESKTOP\Vanompp.lnk"
       Delete "$SMPROGRAMS\Vanompp\Vanompp.lnk"
       Delete "$SMPROGRAMS\Vanompp\Uninstall.lnk"
       RMDir "$SMPROGRAMS\Vanompp"
       DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Vanompp"
       DeleteRegKey HKLM "Software\Vanompp"
       # leave www intentional, inform user
       ExecShell "open" "$INSTDIR"
       Goto done
     nokeep:
       RMDir /r "$INSTDIR"
       Delete "$DESKTOP\Vanompp.lnk"
       RMDir /r "$SMPROGRAMS\Vanompp"
       DeleteRegKey ...
     done:
   SectionEnd
   ```
   Handles existing install: If $INSTDIR\www exists, MessageBox "Folder sudah ada project, timpa bin tapi simpan www/?".

3. `scripts/build-installer.ps1` — new, responsible for producing setup.exe from portable folder:
   ```powershell
   param([string]$Version="0.1.0")
   if (!(Test-Path "dist/Vanompp-portable/Vanompp.exe")) { throw "Portable not built, run build-portable-zip.ps1 first" }
   if (!(Get-Command makensis -ErrorAction SilentlyContinue)) { throw "NSIS not found, winget install NSIS --silent" }
   makensis /DVERSION=$Version installer/installer.nsi
   # output dist/Vanompp_<version>_x64-setup.exe
   ```

4. `src-tauri/tauri.conf.json` — V2 no change needed for bundle since custom NSIS outside, but optionally set `"bundle": {"active": true, "targets": ["msi","nsis"]}` disabled or `[]` to avoid double installer; decision: keep `[]` for V2 portable-only build via `cargo build --release`, installer via custom script. If user wants tauri nsis too, set `installMode: "both"`, `displayLanguageSelector: false`, custom `template` path to override InstallDir to `C:\Vanompp` — but B Hybrid prefers outside to keep portable safe.

5. `GURU_SMK.txt` updated with dual wording: portable extract vs installer next-next.

## 5. Data Flow & Error Handling

**Install Flow (end user clicks setup.exe)**
1. Download from vanompp.my.id → `Vanompp_0.1.0_x64-setup.exe` (~252MB)
2. Double-click → UAC Admin prompt (RequestExecutionLevel admin) — needed for `C:\Vanompp` write + optional vc_redist install
3. NSIS Welcome page: "This will install Vanompp to C:\Vanompp like Laragon portable, with shortcuts"
4. Directory page: default `C:\Vanompp`, Browse button allows `D:\Vanompp`, `E:\Tools\Vanompp` etc. Validate disk free >600MB (exe 15 + bins 250 + temp 250 + overhead), if <600MB MessageBox abort with "Butuh 600MB kosong"
5. If target `www/` or `bin/mysql/data/` exists → MessageBox "Folder sudah ada project & DB, timpa bin tapi simpan www/? Ya/Batal" — Ya continues keeping www/data, Batal aborts
6. InstFiles: `File /r ..\dist\Vanompp-portable\*.*` extraction progress bar, then VC++ check: read reg `HKLM\SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64\Installed=1`, if missing and `vc_redist.x64.exe` bundled, `ExecWait vc_redist /quiet /norestart`
7. Finish page: checkbox Run Vanompp checked by default + show shortcuts created
8. Run: launches `C:\Vanompp\Vanompp.exe` window <2s → Phase1-3 UI

**Uninstall Flow**
1. Via Start Menu `Uninstall.lnk` or `C:\Vanompp\Uninstall.exe` or Add/Remove Programs entry
2. Prompt YESNO per user answer: `Simpan www/ & DB? Ya = keep tugas` 
3. YES path: deletes exe, Uninstall.exe, bin/apache, bin/php, bin/phpmyadmin, logs, vanompp.log, but keeps `www/` (user projects), `bin/mysql/data/` (databases), `vanompp.json` (port config). Deletes shortcuts. Opens Explorer `$INSTDIR` to show kept folders, MessageBox "Project disimpan di C:\Vanompp\www — hapus manual kalau sudah backup"
4. NO path: `RMDir /r $INSTDIR` full, deletes shortcuts, registry keys
5. Either cleans registry Uninstall entry

**Portable Flow (unchanged, safe)**
1. Download zip 250MB → extract anywhere `D:\Vanompp`, `C:\Users\...\Desktop\Vanompp`, flashdisk `E:\Vanompp`
2. Double-click `Vanompp.exe` → same `paths.rs exe.parent` logic → `www/` beside exe auto created → same services start
3. No shortcuts, no registry, no admin, no VC++ auto — if Apache fails due to VCRUNTIME140.dll missing, ServiceCard shows toast "Apache gagal start: VC++ Redist belum install [Download]" button opens https://aka.ms/vs/17/release/vc_redist.x64.exe
4. Delete folder = uninstall (portable)

**Error Cases**
- No admin rights when installing to C:\ → NSIS "RequestExecutionLevel admin" triggers UAC, if user cancels, prompt "Pilih folder yang bisa ditulis tanpa admin, misal D:\Vanompp"
- Disk full <600MB → abort with message
- Antivirus quarantines `mysqld.exe` / `httpd.exe` → installer includes `README_ANTIVIRUS.txt` note: add exception for `C:\Vanompp\bin\...`, portable same
- Existing Laragon/XAMPP using 8080/3306 → not installer concern, runtime `PortConflictModal` Phase3 handles "Port 8080 sudah dipakai coba 8081? Ya ganti"
- Corrupt zip/setup (download fail) → NSIS CRC check auto fails
- MSVC linker missing on dev machine → Phase1-2 can `npm run dev` browser mode (1420) without cargo; Phase4 build to exe requires rustup + MSVC minimal components `MSVC v143 x64/x86 + Windows 11 SDK` ~1.2GB not 4GB full, per Q&A earlier
- Cargo metadata program not found → rustup not in PATH, fix `winget install Rustlang.Rustup`

## 6. Testing

**Manual Checklist V2 (both artifacts from same exe)**
1. Clean Windows VM no prior Vanompp/Laragon/XAMPP, no VC++ Redist — test portable first:
   a. Download portable zip, extract to `D:\Vanompp-portable-test`, double-click exe → window opens <2s → ServiceCard mock ON/OFF → EmptyState CTA
   b. Start All should fail Start if bins missing (if V1 without bins) — expected "Apache belum dibundle, jalankan download-binaries"
   c. With bins (Phase3 Task 3.1 download-binaries.ps1), Start All → Apache ON :8080, MySQL ON :3306, curl `http://localhost:8080/__vano_health` 200 ok, `/phpmyadmin` 200
   d. Create `belajar-php` with DB checked → `www/belajar-php` has index.php, conn.php, .gitignore → BIG URL → Buka di Browser → H1 jalan + DB Connected, phpMyAdmin shows db `belajar_php`
2. Clean VM fresh — test installer:
   a. Run `Vanompp_0.1.0_x64-setup.exe` → UAC → Welcome → default `C:\Vanompp` → InstFiles → VC++ auto install if missing → Finish Run checked → exe launches same as portable
   b. Verify shortcuts: Desktop Vanompp.lnk exists target `C:\Vanompp\Vanompp.exe`, Start Menu Vanompp folder has Vanompp + Uninstall
   c. Repeat create project test same as portable, ensure www path `C:\Vanompp\www\belajar-php`
3. Test custom dir: installer Browse to `D:\MyVanompp` → installs there → exe parent logic works `D:\MyVanompp\www`
4. Test existing install overwrite: reinstall to same `C:\Vanompp` where `www/belajar-php` exists → prompt keep www? Yes → old project preserved after reinstall, bins overwritten fresh
5. Test uninstall keep: Uninstall → Prompt save? Click Ya → after uninstall, `C:\Vanompp\www\belajar-php` still exists, `bin/mysql/data/` exists, shortcuts gone, Explorer opened showing kept folder
6. Test uninstall delete: Reinstall then uninstall → Prompt save? Click Tidak → `C:\Vanompp` fully gone
7. Test disk full, no admin, antivirus note manual
8. Web vanompp.my.id dual download page visual — two buttons same version number, size labels ~250MB each

**Automated**
- cargo test paths slug port rendering template still passes
- `npm run build` Vite 57kB passes
- `svelte-check` / `cargo check` passes
- No Playwright V1 per spec, manual-first
- Git status after build: `dist/` ignored, `src-tauri/resources/bin/` ignored, `installer/installer.nsi` tracked, `scripts/build-installer.ps1` tracked

**Distribution Verification**
- Portable zip when extracted size ~300MB unpacked (~250MB zip)
- Installer exe size ~252MB (zip + NSIS stub 2MB)
- Both produce same file hash for `Vanompp.exe` (built once)
- `vanompp.my.id` download page offers two files same version: `Vanompp-portable-v0.1.0.zip` + `Vanompp_0.1.0_x64-setup.exe`
- GURU_SMK.txt includes both instructions: portable extract vs installer Next Next.

## 7. Decisions Locked

1. **B Hybrid confirmed** — portable folder source of truth, installer wraps it via `File /r`. Keeps portable safe, code unchanged, answers user Q portable aman.
2. **InstallDir `C:\Vanompp` not Program Files** — Laragon-style, keeps `www/` beside exe writable, `paths.rs exe.parent` works both, no AppData split. User can browse to D:.
3. **Shortcuts Desktop + StartMenu** both, per user answer first Q.
4. **Uninstall prompt save www/data Ya/Hapus**, per user answer second Q, prevents tugas hilang.
5. **Admin UAC + auto VC++ Redist for installer**, manual toast for portable — best SMK anti-bingung (decision pending final confirm but recommended). Installer bundles vc_redist.x64.exe /quiet.
6. **NSIS not MSI/WIX** — smaller, Laragon-style, easier custom Keep-www dialog vs WIX.
7. **No change to Rust paths/slug/port/services logic for V2** — packaging only.
8. **Dual build pipeline** — `build-portable-zip.ps1` → `dist/Vanompp-portable/` → zip + setup.exe both from same folder.

## 8. Open Risks

- Windows Defender SmartScreen flags NSIS setup.exe unsigned (~252MB) — mitigation: note on website "Allow" or future code signing cert
- MySQL data 200MB makes download heavy for slow SMK wifi — mitigation: keep zip, future V2 lite without bin (download bins on first Start) optional but out-of-scope now
- Installing to C:\ requires admin which school lab PCs may not have — fallback portable zip no admin works, so dual option covers it (user answer C already handles)
- NSIS not installed on dev machine — `winget install NSIS` one-liner, scripts check existence

## Appendix A: File List V2

```
.
├── installer/
│   └── installer.nsi (new V2, Laragon-style, Desktop shortcut, save-www dialog)
├── scripts/
│   ├── download-binaries.ps1 (Phase3 Task 3.1 existing, downloads Apache/PHP/MySQL/phpMyAdmin ~250MB to resources/bin/)
│   ├── build-portable-zip.ps1 (V1, now refined, produces dist/Vanompp-portable/ + zip)
│   └── build-installer.ps1 (new V2, makensis installer.nsi, requires portable built)
├── src-tauri/
│   ├── tauri.conf.json (bundle.targets = [] or keeps exe only, custom NSIS outside)
│   ├── icons/icon.ico
│   ├── resources/bin/ (gitignored, 15+32+236+14=~297MB, copied by zip script)
│   └── src/{utils/paths.rs (exe.parent same), services/, projects/ etc} (no change)
├── src/ (Svelte 5 + BezelCard etc) (no change V2)
├── www/.gitkeep + __vano_health/index.php (skeleton)
├── dist/
│   ├── Vanompp-portable/ (generated)
│   ├── Vanompp-portable-v0.1.0.zip (generated, portable)
│   └── Vanompp_0.1.0_x64-setup.exe (generated, installer, NSIS)
└── GURU_SMK.txt (updated dual instructions)
```

## Appendix B: Website vanompp.my.id Download Section Draft

```
H1: Vanompp - Localhost Anti-Bingung buat SMK

Pilih cara download (dua-duanya sama, isinya Vanompp.exe + www + bin):

[Button Primary] Download Installer (.exe, 252MB) — Next Next Finish
  → Install ke C:\Vanompp (bisa ganti ke D:\Vanompp saat install)
  → Bikin Shortcut Desktop + Start Menu
  → Butuh Admin sekali + auto install VC++ Redist kalau belum ada
  → Cocok buat PC pribadi / lab yang boleh install

[Button Secondary] Download Portable (.zip, 250MB) — No Install
  → Extract di mana aja: D:\, Desktop, Flashdisk
  → Double-click Vanompp.exe langsung jalan
  → Gak perlu Admin, gak nulis registry
  → Cocok buat sekolah yang gak boleh install / bawa flashdisk
  → Kalau Apache gagal start, klik [Download VC++] di toast (manual 24MB)

Di bawah: hash SHA256 kedua file, ukuran, versi v0.1.0, link GURU_SMK.txt
```

---

**Self-Review Pass:**
- Placeholders? No TBD, all versions, paths, sizes filled (~250MB, C:\Vanompp, 600MB free check, NSIS commands)
- Internal consistency? Yes, B Hybrid keeps portable safe aligns with Q portable aman, dual option C, shortcut Desktop+StartMenu + save prompt answers, paths.rs exe.parent same logic referenced S1-4, bundle no AppData split
- Scope? Single V2 distribution spec, not mixing Phase2/3 logic, isolated to packaging
- Ambiguity? InstallDir C:\Vanompp browsable, admin UAC yes, VC++ auto for installer manual for portable decided, uninstall keep vs delete via YESNO dialog explicit, NSIS vs MSI explicit, portable source of truth explicit

Spec ready for user review → then writing-plans for implementation.
