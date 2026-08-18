param(
  [switch]$Force
)

$ErrorActionPreference = "Stop"
$Base = Resolve-Path "$PSScriptRoot/../src-tauri/resources/bin" -ErrorAction SilentlyContinue
if (-not $Base) {
  New-Item -ItemType Directory -Force -Path "$PSScriptRoot/../src-tauri/resources/bin" | Out-Null
  $Base = Resolve-Path "$PSScriptRoot/../src-tauri/resources/bin"
}
$Base = $Base.Path
$RootZips = Resolve-Path "$PSScriptRoot/.." | Select-Object -ExpandProperty Path

# URLs - latest versions matching local cached files
# Fallbacks for MySQL (Oracle 403 in CI): cdn.mysql.com + GitHub mirror
$Urls = @{
  apache     = "https://www.apachelounge.com/download/VS17/binaries/httpd-2.4.66-251206-Win64-VS17.zip"
  php        = "https://windows.php.net/downloads/releases/php-8.3.33-Win32-vs16-x64.zip"
  mysql      = "https://dev.mysql.com/get/Downloads/MySQL-8.0/mysql-8.0.40-winx64.zip"
  phpmyadmin = "https://files.phpmyadmin.net/phpMyAdmin/5.2.1/phpMyAdmin-5.2.1-all-languages.zip"
}
$FallbackUrls = @{
  mysql = @(
    "https://cdn.mysql.com/Downloads/MySQL-8.0/mysql-8.0.40-winx64.zip"
    "https://downloads.mysql.com/archives/get/p/23/file/mysql-8.0.40-winx64.zip"
  )
}

# Local cached zip paths (fast path if user already downloaded)
$LocalZips = @{
  apache     = Join-Path $RootZips "httpd-2.4.66-251206-Win64-VS17.zip"
  php        = Join-Path $RootZips "php-8.3.33-Win32-vs16-x64.zip"
  mysql      = Join-Path $RootZips "mysql-8.0.40-winx64.zip"
  phpmyadmin = Join-Path $RootZips "phpMyAdmin-5.2.1-all-languages.zip"
}

$Versions = @{
  apache     = "2.4.66 VS17 Win64 (ApacheLounge 251206)"
  php        = "8.3.33 TS Win32-vs16-x64"
  mysql      = "8.0.40 Win64"
  phpmyadmin = "5.2.1-all-languages"
}

# Expected inner folder names inside each zip
$InnerFolders = @{
  apache     = "Apache24"
  php        = $null  # flat
  mysql      = "mysql-8.0.40-winx64"
  phpmyadmin = "phpMyAdmin-5.2.1-all-languages"
}

function Get-TempDir {
  $t = Join-Path ([IO.Path]::GetTempPath()) "vanompp_dl_$(Get-Random)"
  New-Item -ItemType Directory -Force -Path $t | Out-Null
  return $t
}

function Ensure-SourceZip {
  param([string]$Key)
  $local = $LocalZips[$Key]
  $url   = $Urls[$Key]
  $dest  = Join-Path $env:TEMP "vanompp_$Key.zip"

  if (Test-Path $local) {
    Write-Host "[cache] $Key -> $local" -ForegroundColor Green
    return $local
  }

  if ((Test-Path $dest) -and -not $Force) {
    Write-Host "[cache-temp] $Key -> $dest" -ForegroundColor Cyan
    return $dest
  }

  Write-Host "[download] $Key from $url ..." -ForegroundColor Yellow
  try {
    # Use BITS or Invoke-WebRequest with progress disabled for speed
    $ProgressPreference = 'SilentlyContinue'
    # ApacheLounge blocks default UA, must set browser UA + headers
    $headers = @{
      "User-Agent" = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
      "Referer" = "https://www.apachelounge.com/download/"
      "Accept" = "*/*"
    }
    $candidates = @($url)
    if ($FallbackUrls.ContainsKey($Key)) { $candidates += $FallbackUrls[$Key] }
    $lastErr = $null
    foreach ($tryUrl in $candidates) {
      if ($tryUrl -ne $url) { Write-Host "[retry fallback] $Key from $tryUrl ..." -ForegroundColor Yellow }
      try {
        Invoke-WebRequest -Uri $tryUrl -OutFile $dest -UseBasicParsing -TimeoutSec 600 -AllowInsecureRedirect -MaximumRedirection 5 -Headers $headers
        $lastErr = $null
        break
      } catch {
        $lastErr = $_
        Write-Host "[warn fallback] $Key $tryUrl failed: $($_.Exception.Message)" -ForegroundColor DarkYellow
        if ($tryUrl -eq $candidates[-1]) { throw $lastErr }
        Start-Sleep -Seconds 2
        continue
      }
    }
    if ($lastErr) { throw $lastErr }
    $size = (Get-Item $dest).Length / 1MB
    if ((Get-Item $dest).Length -lt 1MB) {
      $sample = Get-Content -Path $dest -TotalCount 3 -ErrorAction SilentlyContinue | Out-String
      Write-Host "[FAIL] $Key zip too small ($([math]::Round($size,3)) MB), likely HTML error page. Sample: $sample" -ForegroundColor Red
      Remove-Item $dest -Force -ErrorAction SilentlyContinue
      throw "Download returned HTML, not zip — blocked or 404"
    }
    Write-Host "[ok] $Key downloaded $([math]::Round($size,1)) MB -> $dest" -ForegroundColor Green
    return $dest
  } catch {
    $msg = $_.Exception.Message
    Write-Host "[FAIL] $Key download failed: $msg" -ForegroundColor Red
    Write-Host "  URL may be 404/outdated - check:" -ForegroundColor Yellow
    if ($Key -eq "apache")   { Write-Host "  https://www.apachelounge.com/download/ for latest httpd VS17 zip" }
    if ($Key -eq "php")      { Write-Host "  https://windows.php.net/downloads/releases/ for php-8.3.x TS x64" }
    if ($Key -eq "mysql")    { Write-Host "  https://dev.mysql.com/downloads/mysql/ for MySQL 8.0 archive" }
    if ($Key -eq "phpmyadmin") { Write-Host "  https://www.phpmyadmin.net/files/ for phpMyAdmin releases" }
    Write-Host "  Place the zip manually in D:\Vanompp root and rerun, or update URL in this script." -ForegroundColor Yellow
    throw
  }
}

function Install-Bin {
  param([string]$Key)

  $target = Join-Path $Base $Key
  if ((Test-Path $target) -and -not $Force) {
    Write-Host "[skip] $Key already exists at $target (use -Force to re-extract)" -ForegroundColor DarkGray
    return
  }

  $zipPath = Ensure-SourceZip -Key $Key
  $tmpExtract = Get-TempDir

  try {
    Write-Host "[extract] $Key $zipPath -> $tmpExtract ..." -ForegroundColor Cyan
    Expand-Archive -Path $zipPath -DestinationPath $tmpExtract -Force

    # Remove old target if Force
    if (Test-Path $target) { Remove-Item $target -Recurse -Force }

    $inner = $InnerFolders[$Key]
    if ($null -ne $inner) {
      $src = Join-Path $tmpExtract $inner
      if (-not (Test-Path $src)) {
        # fallback: if inner not found, look for first directory that looks like the package
        $candidate = Get-ChildItem $tmpExtract -Directory | Select-Object -First 1
        if ($candidate) { $src = $candidate.FullName } else { $src = $tmpExtract }
      }
      # Move src -> target
      Write-Host "[move] $src -> $target" -ForegroundColor Cyan
      # Use Copy then delete tmp to avoid cross-volume issues
      Copy-Item $src -Destination $target -Recurse -Force
    } else {
      # Flat zip (PHP) -> copy tmp contents to target
      New-Item -ItemType Directory -Force -Path $target | Out-Null
      Get-ChildItem $tmpExtract -Force | ForEach-Object {
        Copy-Item $_.FullName -Destination $target -Recurse -Force
      }
    }

    # Write version.txt
    $verFile = Join-Path $target "version.txt"
    $verContent = @"
$Key $($Versions[$Key])
Source: $($Urls[$Key])
LocalZip: $zipPath
Installed: $(Get-Date -Format o)
"@
    Set-Content -Path $verFile -Value $verContent -Encoding utf8

    # Validation
    switch ($Key) {
      "apache" {
        $httpd = Join-Path $target "bin/httpd.exe"
        if (-not (Test-Path $httpd)) { Write-Host "[warn] $httpd not found - unexpected layout!" -ForegroundColor Red }
        else { Write-Host "[ok] apache httpd.exe OK $((Get-Item $httpd).Length/1KB) KB" -ForegroundColor Green }
      }
      "php" {
        $phpExe = Join-Path $target "php.exe"
        $tsDll  = Join-Path $target "php8apache2_4.dll"
        if (-not (Test-Path $phpExe)) { Write-Host "[warn] php.exe missing" -ForegroundColor Red }
        if (-not (Test-Path $tsDll))  { Write-Host "[FAIL] php8apache2_4.dll missing - you need Thread Safe (TS) build!" -ForegroundColor Red }
        else { Write-Host "[ok] php TS dll found: $tsDll" -ForegroundColor Green }
      }
      "mysql" {
        $mysqld = Join-Path $target "bin/mysqld.exe"
        if (-not (Test-Path $mysqld)) { Write-Host "[warn] bin/mysqld.exe missing" -ForegroundColor Red }
        else { Write-Host "[ok] mysql mysqld.exe OK $((Get-Item $mysqld).Length/1MB) MB" -ForegroundColor Green }
      }
      "phpmyadmin" {
        $idx = Join-Path $target "index.php"
        if (-not (Test-Path $idx)) { Write-Host "[warn] phpmyadmin index.php missing" -ForegroundColor Red }
        else { Write-Host "[ok] phpmyadmin index.php OK" -ForegroundColor Green }
      }
    }

    $sizeMB = (Get-ChildItem $target -Recurse -File -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum).Sum / 1MB
    Write-Host "[done] $Key installed $target ($([math]::Round($sizeMB,1)) MB)" -ForegroundColor Green

  } finally {
    Remove-Item $tmpExtract -Recurse -Force -ErrorAction SilentlyContinue
  }
}

Write-Host "Base: $Base" -ForegroundColor White
Write-Host "Root zips check: $RootZips" -ForegroundColor White

$all = @("apache","php","mysql","phpmyadmin")
foreach ($k in $all) { Install-Bin -Key $k }

Write-Host ""
Write-Host "All bins ready. Sizes:" -ForegroundColor Green
Get-ChildItem $Base -Directory | ForEach-Object {
  $sz = (Get-ChildItem $_.FullName -Recurse -File -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum).Sum
  $mb = if ($sz) { [math]::Round($sz/1MB,1) } else { 0 }
  Write-Host ("  {0,-12} {1,8} MB  {2}" -f $_.Name, $mb, $_.FullName)
}

Write-Host ""
Write-Host "Next: cargo run / tauri dev will use these via conf templates." -ForegroundColor DarkGray
