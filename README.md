
# ⚡ opn

**The lightning-fast, fuzzy-matching CLI application launcher for Windows.**

---

<div align="center">
  <img src="https://img.shields.io/badge/Language-Rust-orange?style=for-the-badge&logo=rust" />
  <img src="https://img.shields.io/badge/Platform-Windows-blue?style=for-the-badge&logo=windows" />
  <img src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge" />
</div>

<br />

`opn` is a high-performance utility designed for developers and power users who live in the terminal. Stop searching through the Start Menu—just type a few letters and launch your tools instantly.

[**Explore Releases**](https://github.com/Adarsh-codesOP/op-cli/releases) • [**Report Bug**](https://github.com/Adarsh-codesOP/opn-cli/issues)

---

## 💎 Why use `opn`?

* **🚀 Zero Latency**: Written in Rust. It launches faster than you can hit Enter.
* **🧠 Brainy Search**: Uses fuzzy logic to find `vsc` → `Visual Studio Code`.
* **📈 Self-Optimizing**: It learns. The more you use it, the better it predicts your shortcuts.
* **📦 Portable**: A single binary. No background services, no registry bloat.

---

## 📥 Installation

### **The Fast Way**
1. Download `op.exe` from [Releases](https://github.com/Adarsh-codesOP/op-cli/releases).
2. Drop it into a folder in your system `PATH` (e.g., `C:\bin\`).

### **The Developer Way**
```powershell
git clone [https://github.com/Adarsh-codesOP/opn-cli](https://github.com/Adarsh-codesOP/opn-cli)
cd opn-cli/op
cargo build --release

```

---

## 🎮 How to Use

### **1. Instant Launch**

Launch anything by typing the closest match to its name.

```powershell
op chro   # Opens Chrome
op code   # Opens VS Code
op spot   # Opens Spotify

```

### **2. Teaching the Launcher**

If a query is ambiguous, use the `--select` (or `-s`) flag to pick your favorite. `opn` will remember this choice for next time.

```powershell
op code -s

```

**Interactive Output:**

> **Select an application for 'code':**
> 1. **Visual Studio Code** `(C:\...)`
> 2. **CodeWriter** `(C:\...)`
> 
> 
> **Select [1-2]:** 1
> ✨ *Preference saved!*

---

## ⚙️ Configuration

`opn` keeps its data footprint minimal. Everything is stored in:
`%LOCALAPPDATA%\op\`

| File | Description |
| --- | --- |
| `preferences.json` | Your custom shortcuts and learned behavior. |
| `cache.json` | Indexed application paths for speed. |

> [!TIP]
> To "factory reset" the tool or force a full app re-scan, simply delete the `%LOCALAPPDATA%\op` folder.

---

## 🛠 Contributing

Contributions make the open-source community an amazing place to learn, inspire, and create.

1. **Fork** the Project
2. **Create** your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. **Commit** your Changes (`git commit -m 'Add some AmazingFeature'`)
4. **Push** to the Branch (`git push origin feature/AmazingFeature`)
5. **Open** a Pull Request

---

<p align="center">Made with ❤️ by Adarsh</p>

