# MIT License - Copyright (c) 2026 Destroyer
# Chocolatey uninstall script for disc — Modern DSK image manipulation tool for Amstrad CPC

$ErrorActionPreference = 'Stop'

$packageName = 'disc-image-studio'
$toolsDir    = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

# Remove the binary; Chocolatey removes the shim automatically.
$exePath = Join-Path $toolsDir 'disc.exe'
if (Test-Path $exePath) {
    Remove-Item $exePath -Force
    Write-Host "$packageName has been uninstalled."
}
