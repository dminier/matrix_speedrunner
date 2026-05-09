# Build a Windows release binary natively from PowerShell and package it into
# dist\. À lancer depuis la racine du projet ou depuis scripts\.
#
# Pré-requis :
#   - rustup + Rust toolchain (https://rustup.rs/)
#   - Visual Studio Build Tools 2022 (workload « Outils de génération C++ »)
#     OU MSYS2 + MinGW-w64 si tu préfères la toolchain GNU.

$ErrorActionPreference = "Stop"

$root = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $root

$Name    = "matrix_speedrunner"
$Version = (Select-String -Path "Cargo.toml" -Pattern '^version\s*=\s*"([^"]+)"').Matches[0].Groups[1].Value
$Target  = "x86_64-pc-windows-msvc"
$Dist    = "dist"
$Pkg     = "$Name-$Version-windows-x86_64"

Write-Host "==> Building $Name $Version for $Target"
cargo build --release --target $Target
if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }

$Bin = "target\$Target\release\$Name.exe"
if (-not (Test-Path $Bin)) { throw "Binary not found at $Bin" }

Write-Host "==> Packaging into $Dist\$Pkg.zip"
New-Item -ItemType Directory -Force -Path $Dist | Out-Null
$Stage = Join-Path $Dist $Pkg
if (Test-Path $Stage) { Remove-Item -Recurse -Force $Stage }
New-Item -ItemType Directory -Path $Stage | Out-Null

Copy-Item $Bin       (Join-Path $Stage "$Name.exe")
Copy-Item README.md  $Stage

@'
@echo off
REM Lance Matrix Speedrunner. Conseillé d'ouvrir Windows Terminal pour la
REM pluie en truecolor.
"%~dp0matrix_speedrunner.exe"
pause
'@ | Set-Content -Encoding ASCII (Join-Path $Stage "run.bat")

$Zip = Join-Path $Dist "$Pkg.zip"
if (Test-Path $Zip) { Remove-Item -Force $Zip }
Compress-Archive -Path $Stage -DestinationPath $Zip

$Hash = Get-FileHash -Algorithm SHA256 $Zip
"$($Hash.Hash.ToLower())  $Pkg.zip" | Set-Content -Encoding ASCII (Join-Path $Dist "$Pkg.zip.sha256")

Remove-Item -Recurse -Force $Stage

Write-Host ""
Write-Host "==> Done"
Get-Item $Zip, "$Zip.sha256" | Format-List FullName, Length
