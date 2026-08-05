<p align="center">
  <img src="public/mascot.png" width="160" height="160" alt="Vano mascot" />
  <br/>
  <b>Vanompp</b> — Apache + MySQL portable yang gampang buat pemula
  <br/>
  <a href="#quickstart"><b>Quickstart</b></a> · <a href="#fitur">Fitur</a> · <a href="#cara-pakai">Cara Pakai</a> · <a href="#tech-stack">Tech Stack</a>
</p>

# Vanompp v1.2.0

Portable dev env buat belajar web. Ga ribet setting path localhost, tinggal klik.

## Quickstart

```bash
git clone https://github.com/Atarvano/vanompp && cd vanompp
npm i
npm run tauri dev
```

Apache, MySQL, PHP 8.3 + phpMyAdmin udah included (Windows x64).

## Fitur

- **Apache + MySQL satu klik** — Start All langsung nyala, Stop All buat matiin semua
- **Bikin project baru** — ketik nama, otomatis jadi folder di www/ plus index.php + conn.php + .gitignore
- **Nama database bebas** — bisa tulis nama DB sendiri pas bikin project atau pas project udah ada
- **Lihat semua project** — list semua folder di www/, klik buat buka di browser, copy link, atau buka folder
- **Port bentrok?** — ada pilihan Tetap pakai atau Ganti ke port lain yang kosong
- **Cek error gampang** — tombol Logs buat lihat kenapa gagal jalan
- **Ditemenin Vano** — mascot gajah yang nemenin belajar

## Cara Pakai

1. Buka Vanompp, nyalain Apache sama MySQL
2. Pilih project yang ada atau bikin baru
3. Klik Buka buat liat di browser, atau Buka Folder buat ngoding
4. Butuh database? Centang Buat Database? dan ketik nama yang kamu mau
5. Kalau error, klik Logs

## Tech Stack

- **Svelte + SvelteKit** — UI reaktif
- **Vite** — build tool
- **TypeScript** — type safety
- **Tailwind CSS** — styling
- **Tauri + Rust** — native app portable

## Files

```
Vanompp_1.2.0_x64-setup.exe 149 MB
Vanompp_1.2.0_x64_en-US.msi 289 MB
```

## Dev

```bash
npm run check   # cek 0 errors
npm run dev     # vite 1420
npm run build   # frontend 89KB
npm run tauri build  # installer msi + exe
```

## License

Non-commercial
