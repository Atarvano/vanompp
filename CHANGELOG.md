# Changelog

## v1.2 — Stitch full light 100% SESUAI screenshot + mascot minimal + README lean

- app.css: light tokens #fdfdf9 body text-zinc-900 selection #E9FF70, scrollbar light #e4e4e7, --color-volt #E9FF70
- index.html: body class bg-[#fdfdf9] text-zinc-900 antialiased selection volt + favicon mascot.png
- BezelCard: white bg-white border zinc-200 rounded-24 p-5 shadow 1px2px 04 — remove dark double-layer
- ServiceCard: ON badge #E9FF70 dot black rounded-full, OFF zinc-100, Start black pill + arrow ↗, Stop text, StopAll left + StartAll black pill right + border-t zinc-100, lastError red-50 light
- ProjectCard: hero cream #FEFCE8 border amber-100 rounded-16 p6-8 mono 18-20px bold centered, label 10px uppercase zinc-500, select white border zinc-200 focus ring volt, Buka Folder text + Buka ↗ border pill + Copy URL black pill, Create DB amber-100 + DB emerald badge
- CreateCard: input h-10 rounded-lg border zinc-200 white focus ring volt, preview zinc-50, checkbox DB, button black pill + Create Project, duplicate import merged
- EmptyState: www/ dashed zinc-200 bg-zinc-50 + cream hero, black pill CTA + plus
- Toast: white rounded-16 shadow 08-24-08, dot lime/red, single-source autoMs 4000, App no double timeout
- PortConflictModal: white border zinc-200 rounded-24, rows zinc-50, Tetap pakai white pill + Ganti ke black pill + Buka log
- LogViewer: white border zinc-200 rounded-24, tabs black active, pre zinc-50 border zinc-200 rounded-xl mono 11px, 120 lines tail
- App.svelte: shell #fdfdf9 max-w-720 centered header mascot 32px rounded-10 ring zinc-200 v0.1.0 pill, wiring refreshProjects(ap) via get + addToast only push + modal heuristic + LogViewer on:toast
- Icons: Vano mascot gajah public/mascot.png 512 158KB + docs/images/mascot-512.png/original + all tauri icons 15 files Vano, tauri.conf.json unchanged, root icon baru.png deleted
- README lean: 75 lines ID SMK-first mascot 160 nav Quickstart Fitur Cara Pakai Stack Dev License + tree + stack + dev
- No Rust 0 lines, no stores logic except import merges, no new dep, svelte-check 0 errors 2 a11y warnings (overlay click)

## v1.1 - 2026-08-05 (bugfix Not Responded + Port Persist)

- Not Responded async spawn_blocking fix
- Port conflict persist vanompp.toml atomic + localStorage + Tetap pakai forever
- Modal Tetap pakai VOLT fullwidth + custom badge + x reset
- Apache false fail AH00455 success marker + MySQL OFF brute taskkill orphan
- Cargo 52 tests pass + svelte-check 0 errors

## v0.1.0 - 2026-08-05

- Portable Windows Apache 8080 + MySQL 3306 + PHP 8.3 + phpMyAdmin + Create Folder/DB + Scan Projects + Bundle 289M msi
