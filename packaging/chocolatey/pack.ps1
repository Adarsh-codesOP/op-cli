$ErrorActionPreference = 'Stop'
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectRoot = Resolve-Path "$scriptDir\..\.."
$opDir = "$projectRoot\op"

Write-Host "Building release binary..." -ForegroundColor Cyan
Set-Location $opDir
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed."
}

$exePath = "$opDir\target\release\opn.exe"
$toolsDir = "$scriptDir\tools"

Write-Host "Copying opn.exe to tools directory..." -ForegroundColor Cyan
Copy-Item -Path $exePath -Destination "$toolsDir\opn.exe" -Force

Set-Location $scriptDir

if (Get-Command "choco" -ErrorAction SilentlyContinue) {
    Write-Host "Packing Chocolatey package..." -ForegroundColor Cyan
    choco pack
} else {
    Write-Warning "Chocolatey (choco) not found. Package structure created, but pack failed."
    Write-Warning "Install Chocolatey or run 'choco pack' manually if appropriate."
}
