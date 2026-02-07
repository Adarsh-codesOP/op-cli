# op - Lightweight Windows Application Launcher

`op` is a high-performance, fuzzy-matching CLI application launcher for Windows. It is designed to be:
- **Fast**: Written in Rust, with instant startup and execution.
- **Smart**: Learns your preferences over time.
- **Minimal**: Zero background services, single binary.

## 🚀 Features
- **Fuzzy Search**: Find apps even with partial or typo-filled queries (`code` -> `Visual Studio Code`).
- **Learning**: Remembers your choices. If you select `Visual Studio Code` for `code`, `op code` will launch it instantly next time.
- **Offline**: Works entirely local, no internet required.
- **Portable**: Single `op.exe` binary, no installation required.

## 📦 Installation
### Manual
1. Download `op.exe` from the [Releases](https://github.com/Adarsh-codesOP/op-cli/releases) page.
2. Place it in a folder included in your system `PATH` (e.g., `C:\Program Files\op\`).

### From Source
```powershell
git clone https://github.com/Adarsh-codesOP/op-cli
cd op-cli/op
cargo build --release
# Binary is at target/release/op.exe
```

## 🛠 Usage

### Launch an App
```powershell
op <query>
# Example:
op chro   # Launches Google Chrome
op code   # Launches Visual Studio Code
```

### Interactive Selection
If multiple apps match or you want to teach `op`, use `-s` or `--select`:
```powershell
op code -s
```
Output:
```
Select an application for 'code':
[1] Visual Studio Code (C:\...)
[2] CodeWriter (C:\...)
> 1
```
`op` will remember that `code` means `Visual Studio Code`. Next time `op code` will launch it directly.

## ⚙️ Configuration & Reset
Preferences are stored in `%LOCALAPPDATA%\op\preferences.json`.
Cache is stored in `%LOCALAPPDATA%\op\cache.json`.

To reset learning or force a re-scan:
```powershell
# Delete the data directory
Remove-Item -Recurse -Force "$env:LOCALAPPDATA\op"
```
The next run will automatically rebuild the cache.

## 🤝 Contributing
Contributions are welcome! Please open an issue or submit a PR.

## 📜 License
MIT License
