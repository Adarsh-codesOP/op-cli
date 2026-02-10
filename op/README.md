# opn - Lightweight Windows Application Launcher

`opn` is a high-performance, fuzzy-matching CLI application launcher for Windows. It is designed to be:
- **Fast**: Written in Rust, with instant startup and execution.
- **Smart**: Learns your preferences over time.
- **Minimal**: Zero background services, single binary.

## 🚀 Features
- **Fuzzy Search**: Find apps even with partial or typo-filled queries (`code` -> `Visual Studio Code`).
- **Learning**: Remembers your choices. If you select `Visual Studio Code` for `code`, `opn code` will launch it instantly next time.
- **Offline**: Works entirely local, no internet required.
- **Portable**: Single `opn.exe` binary, no installation required.

## 📦 Installation
### Chocolatey
```powershell
choco install opn
```

### Manual
1. Download `opn.exe` from the [Releases](https://github.com/Adarsh-codesOP/opn-cli/releases) page.
2. Place it in a folder included in your system `PATH` (e.g., `C:\Program Files\opn\`).

### From Source
```powershell
git clone https://github.com/Adarsh-codesOP/opn-cli
cd opn-cli/op
cargo build --release
# Binary is at target/release/opn.exe
```

## 🛠 Usage

### Launch an App
```powershell
opn <query>
# Example:
opn chro   # Launches Google Chrome
opn code   # Launches Visual Studio Code
```

### Interactive Selection
If multiple apps match or you want to teach `opn`, use `-s` or `--select`:
```powershell
opn code -s
```
Output:
```
Select an application for 'code':
[1] Visual Studio Code (C:\...)
[2] CodeWriter (C:\...)
> 1
```
`opn` will remember that `code` means `Visual Studio Code`. Next time `opn code` will launch it directly.

## ⚙️ Configuration & Reset
Preferences are stored in `%LOCALAPPDATA%\opn\preferences.json`.
Cache is stored in `%LOCALAPPDATA%\opn\cache.json`.

To reset learning or force a re-scan:
```powershell
# Delete the data directory
Remove-Item -Recurse -Force "$env:LOCALAPPDATA\opn"
```
The next run will automatically rebuild the cache.

## 🤝 Contributing
Contributions are welcome! Please open an issue or submit a PR.

## 📜 License
MIT License
