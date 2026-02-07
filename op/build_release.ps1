cargo build --release
if ($LASTEXITCODE -eq 0) {
    Copy-Item target/release/op.exe .
    Write-Host "Build complete: op.exe created in current directory." -ForegroundColor Green
} else {
    Write-Host "Build failed." -ForegroundColor Red
}
