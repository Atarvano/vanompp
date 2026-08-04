# Vanompp Binary Resources

This folder is **gitignored** on purpose – binaries are 300-500 MB extracted.

## Why ignored
`src-tauri/resources/bin/*` contains full Apache, PHP, MySQL, phpMyAdmin.
Committing them bloats git and slows clone. Bundle includes them at build time
via `tauri.conf.json` `bundle.resources`, so portable zip / installer still works.

## Expected layout
```
src-tauri/resources/bin/
  apache/
    bin/httpd.exe
    conf/
    modules/
    logs/ (created at runtime)
    version.txt
  php/
    php.exe
    php8apache2_4.dll  <- MUST exist, proves TS build
    ext/
    tmp/  logs/ (runtime)
    version.txt
  mysql/
    bin/mysqld.exe
    data/ (created on first init, gitignored inside?)
    version.txt
  phpmyadmin/
    index.php
    config.sample.inc.php
    version.txt
  README.md (this file)
```

## Versions (V1 lock)
- Apache 2.4.66 Win64 VS17 from ApacheLounge (httpd-2.4.66-251206-Win64-VS17.zip)
- PHP 8.3.33 TS Win32-vs16-x64 Thread Safe (needs php8apache2_4.dll)
- MySQL 8.0.40 Win64
- phpMyAdmin 5.2.1 all-languages

Total installed ~ 700 MB, zips ~ 300 MB.

## How to download / extract

### Automatic (preferred)
```powershell
# From project root:
.\scripts\download-bin.ps1
# Force re-extract:
.\scripts\download-bin.ps1 -Force
```

Script behavior:
- Checks if `D:\Vanompp\httpd-*.zip`, `php-*.zip`, `mysql-*.zip`, `phpMyAdmin-*.zip` exist in repo root (cached)
- If yes, uses local copy instead of downloading (fast path)
- Falls back to downloading via Invoke-WebRequest
- Flattens zip structure:
  Apache24/* -> apache/
  mysql-8.0.40-winx64/* -> mysql/
  phpMyAdmin-.../* -> phpmyadmin/
  php flat -> php/
- Writes version.txt
- Validates key exes

### Manual
1. Download zips to project root or let script download to %TEMP%.
2. Extract as described above.

### URLs used
- https://www.apachelounge.com/download/VS17/binaries/httpd-2.4.66-251206-Win64-VS17.zip
- https://windows.php.net/downloads/releases/php-8.3.33-Win32-vs16-x64.zip
- https://dev.mysql.com/get/Downloads/MySQL-8.0/mysql-8.0.40-winx64.zip
- https://files.phpmyadmin.net/phpMyAdmin/5.2.1/phpMyAdmin-5.2.1-all-languages.zip

If any URL 404 (ApacheLounge rotates old builds), search:
- Apache: https://www.apachelounge.com/download/
- PHP: https://windows.php.net/downloads/releases/
- MySQL: https://dev.mysql.com/downloads/mysql/ (archive 8.0.x)
- phpMyAdmin: https://www.phpmyadmin.net/files/

Update URLs in scripts/download-bin.ps1.

## Thread Safe vs Non Thread Safe (critical)
Apache mod_php requires **Thread Safe** PHP build.
- TS filename: `php-8.3.33-Win32-vs16-x64.zip` contains `php8apache2_4.dll`
- NTS filename: `php-8.3.33-nts-Win32-vs16-x64.zip` does NOT contain dll, will fail

Script checks for dll and warns.

## Root zip cache
Zips in repo root `D:\Vanompp\*.zip` are NOT gitignored yet but are large.
Recommended: keep them local, or delete after extraction to save disk.
They are safe to delete - script will re-download if needed.

To gitignore root zips add to .gitignore:
```
*.zip
!src-tauri/icons/*
```

Currently only `src-tauri/resources/bin/` is ignored per plan.

## Tauri bundle
tauri.conf.json bundles `resources/bin` + `../www/__vano_health` as resources.
These copy next to exe in portable build, enabling self-contained Vanompp.

## Size note
- mysql-8.0.40-winx64.zip 233 MB extracted ~ 400 MB
- php 32 MB extracted ~ 90 MB
- apache 11 MB extracted ~ 40 MB
- phpMyAdmin 15 MB extracted ~ 70 MB
Total bundle ~ 1.2 GB with Tauri runtime + resources. NSIS compresses ~ 50%.

## VC++ requirement
Apache and PHP need VC++ Redistributable VS17/VS16.
If Start fails, check error.log - usually missing VCRUNTIME140.dll.
User should install https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist
