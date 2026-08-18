<p align="center">
  <img src="public/mascot.png" width="160" height="160" alt="Vano mascot" />
  <br/>
  <b>Vanompp</b> — Portable Apache + MySQL that's easy for beginners
  <br/>
  <a href="#quickstart"><b>Quickstart</b></a> · <a href="#features">Features</a> · <a href="#how-to-use">How to Use</a> · <a href="#tech-stack">Tech Stack</a>
</p>

# Vanompp v1.2.0

Portable dev env for learning web. No localhost path setup. Just click.

## Quickstart

```bash
git clone https://github.com/Atarvano/vanompp && cd vanompp
npm i
npm run tauri dev
```

Apache, MySQL, PHP 8.3 + phpMyAdmin are included (Windows x64).

## Features

- **Apache + MySQL in one click** — Start All turns them on, Stop All turns them off
- **Create a new project** — type a name, it becomes a folder in www/ plus index.php + conn.php + .gitignore
- **Free database name** — write your own DB name when creating a project or after it already exists
- **See all projects** — list every folder in www/, click to open in the browser, copy the link, or open the folder
- **Port conflict?** — keep the current port or switch to the next free one
- **Easy error check** — Logs button shows why a start failed
- **Vano comes along** — elephant mascot that keeps you company while learning

## How to Use

1. Open Vanompp, start Apache and MySQL
2. Pick an existing project or create a new one
3. Click Open to view it in the browser, or Open Folder to code
4. Need a database? Check Create Database? and type the name you want
5. If something errors, click Logs

## Tech Stack

- **Svelte + SvelteKit** — reactive UI
- **Vite** — build tool
- **TypeScript** — type safety
- **Tailwind CSS** — styling
- **Tauri + Rust** — portable native app

## Dev

```bash
npm run check   # check 0 errors
npm run dev     # vite 1420
npm run build   # frontend 89KB
npm run tauri build  # installer msi + exe
```

## License

Non-commercial
