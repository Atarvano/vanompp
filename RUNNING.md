# Running the Sidebar Redesign (v1.3.0)

Cara kerjain tiket-tiket dari #1 spec. Bisa solo (satu session) atau parallel (multi-agent).

---

## Quick Start

### Option A: Solo (satu agent, satu session)

Kerjain urut dari atas:

```bash
# 1. i18n foundation
"Kerjain ticket #2: i18n foundation + locale store"

# 2. Sidebar shell
"Kerjain ticket #3: Sidebar shell + App router"

# 3. Pilih salah satu page (atau semua berturut-turut)
"Kerjain ticket #4: Services page + Terminal tab"
"Kerjain ticket #5: Project page"
"Kerjain ticket #6: Pengembang page"

# 4. Finalize
"Kerjain ticket #7: Version 1.3.0 + antislop audit + E2E test"
```

### Option B: Parallel (multi-agent / multi-session)

Buka 3 terminal / 3 session terpisah:

**Terminal 1 (Agent A):**
```bash
"Kerjain ticket #2: i18n foundation, lanjut ke ticket #3: Sidebar shell"
```

**Terminal 2 (Agent B) — setelah Ticket 3 done:**
```bash
"Kerjain ticket #4: Services page + Terminal tab"
```

**Terminal 3 (Agent C) — setelah Ticket 3 done:**
```bash
"Kerjain ticket #5: Project page"
```

**Terminal 4 (Agent D) — setelah Ticket 3 done:**
```bash
"Kerjain ticket #6: Pengembang page"
```

**Terakhir (siapa aja):**
```bash
"Kerjain ticket #7: Version 1.3.0 + antislop audit + E2E test"
```

---

## Dependency Rules (PENTING)

| Ticket | Butuh | Bisa barengan dengan |
|--------|-------|----------------------|
| #2 i18n | — | — |
| #3 Sidebar | #2 done | — |
| #4 Services | #3 done | #5, #6 |
| #5 Project | #3 done | #4, #6 |
| #6 Pengembang | #3 done | #4, #5 |
| #7 Finalize | #4, #5, #6 done | — |

**Artinya:**
- #4, #5, #6 bisa dikerjain **bersamaan** (3 agent berbeda)
- #7 harus nunggu semua selesai

---

## Prompt Template per Ticket

### Ticket 2: i18n foundation
```
Kerjain ticket #2: i18n foundation + locale store

Scope:
- Buat src/lib/stores/locale.ts (writable<'en' | 'id'>)
- Refactor src/lib/utils/messages.ts untuk support EN/ID
- Buat helper t() untuk translate
- Default 'id', persist ke localStorage
- Semua copy existing ditranslate

Jangan kerjain:
- UI changes (itu ticket #3+)
- Sidebar (ticket #3)

Done when: locale store works, t() bisa dipakai, semua copy ada EN/ID
```

### Ticket 3: Sidebar shell
```
Kerjain ticket #3: Sidebar shell + App router

Scope:
- Buat src/lib/components/Sidebar.svelte
- Rewrite App.svelte dengan sidebar + page router
- 3 nav: Services, Project, Pengembang
- Collapse toggle (200px ↔ 48px)
- Language switcher di sidebar
- Version v1.3.0 di bawah

Jangan kerjain:
- Page content (Services, Project, Pengembang)
- Service/Project logic

Done when: Sidebar renders, routing works, collapse persists, language switcher works
```

### Ticket 4: Services + Terminal
```
Kerjain ticket #4: Services page + Terminal tab

Scope:
- Buat src/lib/components/ServicesPage.svelte
- Tab switcher: Apache | MySQL | Terminal
- Refactor ServiceCard logic ke tabs
- Buat TerminalTab.svelte (MySQL client)
- Rust command spawn_mysql_terminal kalau perlu

Jangan kerjain:
- Project page
- Pengembang page
- Sidebar (udah ada)

Done when: Services page works, Terminal connects to MySQL, Start/Stop works
```

### Ticket 5: Project page
```
Kerjain ticket #5: Project page

Scope:
- Buat src/lib/components/ProjectPage.svelte
- List projects + Create form
- Open Explorer/Browser buttons
- URL card, phpMyAdmin, Create DB
- Refactor ProjectCard + CreateCard

Jangan kerjain:
- Services page
- Pengembang page
- Sidebar (udah ada)

Done when: Project page works, create/list/open works, DB creation works
```

### Ticket 6: Pengembang page
```
Kerjain ticket #6: Pengembang page

Scope:
- Buat src/lib/components/PengembangPage.svelte
- About card (creator, repo, description)
- System Info card (Apache/MySQL status, version)

Jangan kerjain:
- Services page
- Project page
- Sidebar (udah ada)

Done when: Pengembang page renders, info displays, links work
```

### Ticket 7: Finalize
```
Kerjain ticket #7: Version 1.3.0 + antislop audit + E2E test

Scope:
- Verify version 1.3.0 everywhere
- Run antislop Delivery Gate
- E2E test all flows
- Fix issues
- Prepare PR

Done when: All tests pass, no console errors, ready to merge
```

---

## Branch Strategy

Semua tiket dikerjain di branch `prototype/redesign`:

```bash
# Setiap mulai ticket, pastikan di branch yang benar
git checkout prototype/redesign

# Setelah selesai, commit dengan message jelas
git add .
git commit -m "feat: ticket #2 - i18n foundation"
```

Kalau parallel, merge conflict mungkin terjadi di `App.svelte` atau shared files. Resolve manual, terus lanjut.

---

## Tips

- **Jangan skip Ticket 2** — semua butuh i18n
- **Jangan skip Ticket 3** — semua butuh sidebar
- **Ticket 4-6 bisa swap** — tergantung prioritas
- **Ticket 7 wajib terakhir** — butuh semua pages

---

## Checklist Sebelum Mulai

- [ ] Di branch `prototype/redesign`
- [ ] Ticket 2 (i18n) done
- [ ] Ticket 3 (Sidebar) done
- [ ] Pilih ticket 4, 5, atau 6
- [ ] Baca prompt template di atas
