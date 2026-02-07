# Winget Packaging

To publish `op` to Winget (Windows Package Manager), follow these steps:

1.  **Release on GitHub**: Create a new Release in your GitHub repo and upload `op.exe` (or `op.zip`).
2.  **Generate Manifest**: Use the `wingetcreate` tool (install via `winget install wingetcreate`).
    ```powershell
    wingetcreate new https://github.com/Adarsh-codesOP/op-cli/releases/download/v0.1.0/op.exe
    ```
3.  **Submit**: Follow the prompts to generate the YAML manifest and submit a PR to the [winget-pkgs](https://github.com/microsoft/winget-pkgs) repository.

Alternatively, manually create a manifest in this directory using the template from [Microsoft Docs](https://learn.microsoft.com/en-us/windows/package-manager/package/manifest-yaml).
