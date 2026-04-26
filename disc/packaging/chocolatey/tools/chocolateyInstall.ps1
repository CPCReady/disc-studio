# MIT License - Copyright (c) 2026 Destroyer
# Chocolatey install script for disc — Modern DSK image manipulation tool for Amstrad CPC
#
# Replace GITHUB_OWNER with the actual GitHub username/organization.
# Update $checksum64 on every release:
#   (Invoke-WebRequest -Uri $url64 -UseBasicParsing).RawContentStream | Get-FileHash -Algorithm SHA256

$ErrorActionPreference = 'Stop'

$packageName = 'disc-image-studio'
$toolsDir    = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

$url64 = 'https://github.com/GITHUB_OWNER/disc-image-studio/releases/download/v0.1.0/disc-windows-x86_64.exe'
$checksum64 = 'REPLACE_WITH_SHA256_OF_WINDOWS_BINARY'

# Download the pre-built binary and place it in the tools directory.
# Chocolatey automatically creates a shim so `disc` is available on PATH.
$exePath = Join-Path $toolsDir 'disc.exe'

$packageArgs = @{
  packageName   = $packageName
  fileType      = 'exe'
  url64bit      = $url64
  checksum64    = $checksum64
  checksumType64 = 'sha256'
  file64        = $exePath
}

Get-ChocolateyWebFile @packageArgs

# Verify the binary works
$version = & $exePath --version 2>&1
Write-Host "Installed: $version"
