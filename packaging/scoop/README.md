# Scoop Packaging

To publish `op` to Scoop, you need a JSON manifest.

1.  **Release on GitHub**: Upload `op.exe` (or `op.zip`) to GitHub Releases.
2.  **Create Manifest**: Create `op.json` in this directory:
    ```json
    {
        "version": "0.1.0",
        "description": "A lightweight Windows application launcher.",
        "homepage": "https://github.com/Adarsh-codesOP/op-cli",
        "license": "MIT",
        "url": "https://github.com/Adarsh-codesOP/op-cli/releases/download/v0.1.0/op.exe",
        "hash": "<SHA256-HASH-OF-EXE>",
        "bin": "op.exe"
    }
    ```
3.  **Publish**:
    - Commit this file to your own "bucket" repository (e.g., `my-scoop-bucket`).
    - Run `scoop bucket add my-bucket <repo-url>`.
    - Run `scoop install op`.
