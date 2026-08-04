param([string]$Version="0.1.0", [string]$Root="D:\Vanompp")
$DistRoot = Join-Path $Root "dist"
Write-Host "== Vanompp installer check ==" -ForegroundColor Cyan
Write-Host "DistRoot $DistRoot"
$found=0
Get-ChildItem $DistRoot -Recurse -File -Filter "*.msi" -ErrorAction SilentlyContinue | ForEach-Object { $found=1; Write-Host ("[msi] {0} {1} MB {2}" -f $_.Name, [math]::Round($_.Length/1MB,2), $_.FullName) -ForegroundColor Green }
Get-ChildItem $DistRoot -Recurse -File -Filter "*setup*.exe" -ErrorAction SilentlyContinue | ForEach-Object { $found=1; Write-Host ("[nsis] {0} {1} MB" -f $_.Name, [math]::Round($_.Length/1MB,2), $_.FullName) -ForegroundColor Green }
Get-ChildItem $DistRoot -Filter "Vanompp-portable-v*.zip" -File -ErrorAction SilentlyContinue | ForEach-Object { $found=1; Write-Host ("[zip] {0} {1} KB" -f $_.Name, [math]::Round($_.Length/1KB,1)) -ForegroundColor Green }
if ($found -eq 0) { Write-Host "[info] No msi/nsis/zip yet - run portable zip or tauri build" -ForegroundColor Yellow }
Write-Host "Done"
