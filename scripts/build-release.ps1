$ErrorActionPreference = "Stop"

$ProjectRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).ProviderPath
$BuildRoot = Join-Path $env:TEMP "windows-clicker-build"
$SourceRoot = Join-Path $BuildRoot "src"
$DistRoot = Join-Path $ProjectRoot "dist"

if (Test-Path $BuildRoot) {
    Remove-Item -Recurse -Force $BuildRoot
}

New-Item -ItemType Directory -Force $SourceRoot | Out-Null
New-Item -ItemType Directory -Force $DistRoot | Out-Null

Copy-Item (Join-Path $ProjectRoot "Cargo.toml") $SourceRoot
Copy-Item (Join-Path $ProjectRoot "Cargo.lock") $SourceRoot
Copy-Item -Recurse (Join-Path $ProjectRoot "src") $SourceRoot
Copy-Item -Recurse (Join-Path $ProjectRoot "tests") $SourceRoot

Push-Location $SourceRoot
try {
    cargo.exe test
    cargo.exe build --release
    Copy-Item `
        (Join-Path $SourceRoot "target\release\windows-clicker.exe") `
        (Join-Path $DistRoot "windows-clicker.exe") `
        -Force
}
finally {
    Pop-Location
}

Remove-Item -Recurse -Force $BuildRoot

Write-Host "Built $DistRoot\windows-clicker.exe"
