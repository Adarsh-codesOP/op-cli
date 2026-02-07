$ErrorActionPreference = 'Stop'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
# Chocolatey automatically removes shims and the package directory.
# We can add custom cleanup here if needed (e.g. data in LocalAppData), 
# but per standard practice, we usually leave user data properly.
