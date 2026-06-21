param(
    [Parameter(Mandatory = $true)]
    [string] $ExePath,

    [Parameter(Mandatory = $true)]
    [string] $IconPath
)

$ErrorActionPreference = "Stop"

Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;

public static class ResourceUpdater {
    [DllImport("kernel32.dll", SetLastError = true, CharSet = CharSet.Unicode)]
    public static extern IntPtr BeginUpdateResource(string pFileName, bool bDeleteExistingResources);

    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern bool UpdateResource(IntPtr hUpdate, IntPtr lpType, IntPtr lpName, ushort wLanguage, byte[] lpData, uint cbData);

    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern bool EndUpdateResource(IntPtr hUpdate, bool fDiscard);
}
"@

function Read-UInt16 {
    param([byte[]] $Bytes, [int] $Offset)
    return [BitConverter]::ToUInt16($Bytes, $Offset)
}

function Read-UInt32 {
    param([byte[]] $Bytes, [int] $Offset)
    return [BitConverter]::ToUInt32($Bytes, $Offset)
}

$ExePath = (Resolve-Path $ExePath).ProviderPath
$IconPath = (Resolve-Path $IconPath).ProviderPath
$IconBytes = [System.IO.File]::ReadAllBytes($IconPath)

$Reserved = Read-UInt16 $IconBytes 0
$Type = Read-UInt16 $IconBytes 2
$Count = Read-UInt16 $IconBytes 4

if ($Reserved -ne 0 -or $Type -ne 1 -or $Count -lt 1) {
    throw "Invalid ICO file: $IconPath"
}

$Group = New-Object System.Collections.Generic.List[byte]
$Group.AddRange([BitConverter]::GetBytes([uint16]0))
$Group.AddRange([BitConverter]::GetBytes([uint16]1))
$Group.AddRange([BitConverter]::GetBytes([uint16]$Count))

$IconImages = @()
for ($Index = 0; $Index -lt $Count; $Index++) {
    $EntryOffset = 6 + ($Index * 16)
    $Width = $IconBytes[$EntryOffset]
    $Height = $IconBytes[$EntryOffset + 1]
    $ColorCount = $IconBytes[$EntryOffset + 2]
    $ReservedByte = $IconBytes[$EntryOffset + 3]
    $Planes = Read-UInt16 $IconBytes ($EntryOffset + 4)
    $BitCount = Read-UInt16 $IconBytes ($EntryOffset + 6)
    $BytesInRes = Read-UInt32 $IconBytes ($EntryOffset + 8)
    $ImageOffset = Read-UInt32 $IconBytes ($EntryOffset + 12)
    $ResourceId = $Index + 1

    $Image = New-Object byte[] $BytesInRes
    [Array]::Copy($IconBytes, $ImageOffset, $Image, 0, $BytesInRes)
    $IconImages += [pscustomobject]@{
        ResourceId = $ResourceId
        Bytes = $Image
    }

    $Group.Add([byte]$Width)
    $Group.Add([byte]$Height)
    $Group.Add([byte]$ColorCount)
    $Group.Add([byte]$ReservedByte)
    $Group.AddRange([BitConverter]::GetBytes([uint16]$Planes))
    $Group.AddRange([BitConverter]::GetBytes([uint16]$BitCount))
    $Group.AddRange([BitConverter]::GetBytes([uint32]$BytesInRes))
    $Group.AddRange([BitConverter]::GetBytes([uint16]$ResourceId))
}

$Handle = [ResourceUpdater]::BeginUpdateResource($ExePath, $false)
if ($Handle -eq [IntPtr]::Zero) {
    throw "BeginUpdateResource failed for $ExePath"
}

try {
    foreach ($Image in $IconImages) {
        $Ok = [ResourceUpdater]::UpdateResource(
            $Handle,
            [IntPtr]3,
            [IntPtr]$Image.ResourceId,
            0,
            $Image.Bytes,
            [uint32]$Image.Bytes.Length
        )
        if (-not $Ok) {
            throw "UpdateResource RT_ICON failed for resource $($Image.ResourceId)"
        }
    }

    $GroupBytes = $Group.ToArray()
    $Ok = [ResourceUpdater]::UpdateResource(
        $Handle,
        [IntPtr]14,
        [IntPtr]1,
        0,
        $GroupBytes,
        [uint32]$GroupBytes.Length
    )
    if (-not $Ok) {
        throw "UpdateResource RT_GROUP_ICON failed"
    }

    $Ok = [ResourceUpdater]::EndUpdateResource($Handle, $false)
    if (-not $Ok) {
        throw "EndUpdateResource failed"
    }
    $Handle = [IntPtr]::Zero
}
finally {
    if ($Handle -ne [IntPtr]::Zero) {
        [void][ResourceUpdater]::EndUpdateResource($Handle, $true)
    }
}
