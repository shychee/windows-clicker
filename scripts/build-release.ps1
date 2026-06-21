$ErrorActionPreference = "Stop"

$ProjectRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).ProviderPath
$BuildRoot = Join-Path $env:TEMP "windows-clicker-build"
$SourceRoot = Join-Path $BuildRoot "src"
$DistRoot = Join-Path $ProjectRoot "dist"
$AssetRoot = Join-Path $ProjectRoot "assets"
$IconPath = Join-Path $AssetRoot "windows-clicker.ico"

function Invoke-Cargo {
    param(
        [Parameter(Mandatory = $true)]
        [string[]] $Arguments
    )

    & cargo.exe @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "cargo.exe $($Arguments -join ' ') failed with exit code $LASTEXITCODE"
    }
}

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
    Invoke-Cargo @("test", "--lib", "--tests")
    Invoke-Cargo @("build", "--release")
    $OutputExe = Join-Path $SourceRoot "target\release\windows-clicker.exe"
    & (Join-Path $ProjectRoot "scripts\embed-icon.ps1") `
        -ExePath $OutputExe `
        -IconPath $IconPath
    Copy-Item `
        $OutputExe `
        (Join-Path $DistRoot "windows-clicker.exe") `
        -Force
}
finally {
    Pop-Location
}

Remove-Item -Recurse -Force $BuildRoot

Write-Host "Built $DistRoot\windows-clicker.exe"
Get-FileHash (Join-Path $DistRoot "windows-clicker.exe") -Algorithm SHA256 |
    ForEach-Object { Write-Host "SHA256 $($_.Hash)" }
