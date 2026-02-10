cargo build --release
if ($LASTEXITCODE -eq 0) {
    Copy-Item target/release/opn.exe .
    Write-Host "Build complete: opn.exe created in current directory." -ForegroundColor Green
} else {
    Write-Host "Build failed." -ForegroundColor Red
}
