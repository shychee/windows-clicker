param(
    [Parameter(Mandatory = $true)]
    [string] $ExePath
)

$ErrorActionPreference = "Stop"

Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;

public static class IconResourceCheck {
    [DllImport("kernel32.dll", CharSet = CharSet.Unicode, SetLastError = true)]
    public static extern IntPtr LoadLibraryEx(string lpFileName, IntPtr hFile, uint dwFlags);

    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern IntPtr FindResource(IntPtr hModule, IntPtr lpName, IntPtr lpType);

    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern bool FreeLibrary(IntPtr hModule);
}
"@

$ResolvedExe = (Resolve-Path $ExePath).ProviderPath
$Module = [IconResourceCheck]::LoadLibraryEx($ResolvedExe, [IntPtr]::Zero, 2)
if ($Module -eq [IntPtr]::Zero) {
    throw "LoadLibraryEx failed"
}

try {
    $Resource = [IconResourceCheck]::FindResource($Module, [IntPtr]1, [IntPtr]14)
    if ($Resource -eq [IntPtr]::Zero) {
        throw "icon group missing"
    }
    Write-Host "icon group present"
}
finally {
    [void][IconResourceCheck]::FreeLibrary($Module)
}
