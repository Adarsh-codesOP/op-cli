$ErrorActionPreference = 'Stop'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

# The binary is already in the package tools directory, so Chocolatey handles the shim.
# We just need to ensure it's there.
# If we were downloading from a URL, we'd do it here. But for a self-contained package:
# just ensuring the exe is present is enough.

Write-Host "opn installed successfully. Run 'opn' to launch applications."
