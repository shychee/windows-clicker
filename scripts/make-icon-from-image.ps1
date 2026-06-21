param(
    [Parameter(Mandatory = $true)]
    [string] $InputPath,

    [Parameter(Mandatory = $true)]
    [string] $OutputPath
)

$ErrorActionPreference = "Stop"

Add-Type -AssemblyName System.Drawing

function New-PngBytes {
    param(
        [System.Drawing.Image] $Source,
        [int] $Size
    )

    $Bitmap = New-Object System.Drawing.Bitmap $Size, $Size, ([System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $Graphics = [System.Drawing.Graphics]::FromImage($Bitmap)
    try {
        $Graphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
        $Graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
        $Graphics.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
        $Graphics.Clear([System.Drawing.Color]::Transparent)

        $Path = New-Object System.Drawing.Drawing2D.GraphicsPath
        $Path.AddEllipse(0, 0, $Size, $Size)
        $Graphics.SetClip($Path)
        $Graphics.DrawImage($Source, 0, 0, $Size, $Size)
        $Graphics.ResetClip()

        $PenWidth = [Math]::Max(2, [int]($Size / 16))
        $Pen = New-Object System.Drawing.Pen ([System.Drawing.Color]::FromArgb(255, 26, 32, 42)), $PenWidth
        $Graphics.DrawEllipse($Pen, $PenWidth / 2, $PenWidth / 2, $Size - $PenWidth, $Size - $PenWidth)

        $Stream = New-Object System.IO.MemoryStream
        $Bitmap.Save($Stream, [System.Drawing.Imaging.ImageFormat]::Png)
        return $Stream.ToArray()
    }
    finally {
        $Graphics.Dispose()
        $Bitmap.Dispose()
    }
}

function Add-UInt16 {
    param([System.Collections.Generic.List[byte]] $Bytes, [int] $Value)
    foreach ($Byte in [BitConverter]::GetBytes([uint16]$Value)) {
        $Bytes.Add($Byte)
    }
}

function Add-UInt32 {
    param([System.Collections.Generic.List[byte]] $Bytes, [int] $Value)
    foreach ($Byte in [BitConverter]::GetBytes([uint32]$Value)) {
        $Bytes.Add($Byte)
    }
}

$InputPath = (Resolve-Path $InputPath).ProviderPath
$OutputPath = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($OutputPath)
$OutputDir = Split-Path -Parent $OutputPath
New-Item -ItemType Directory -Force $OutputDir | Out-Null

$Source = [System.Drawing.Image]::FromFile($InputPath)
try {
    $Sizes = @(16, 32, 48, 256)
    $Images = foreach ($Size in $Sizes) {
        [pscustomobject]@{
            Size = $Size
            Bytes = New-PngBytes -Source $Source -Size $Size
        }
    }

    $Output = New-Object System.Collections.Generic.List[byte]
    Add-UInt16 $Output 0
    Add-UInt16 $Output 1
    Add-UInt16 $Output $Images.Count

    $Offset = 6 + (16 * $Images.Count)
    foreach ($Image in $Images) {
        $EncodedSize = if ($Image.Size -eq 256) { 0 } else { $Image.Size }
        $Output.Add([byte]$EncodedSize)
        $Output.Add([byte]$EncodedSize)
        $Output.Add([byte]0)
        $Output.Add([byte]0)
        Add-UInt16 $Output 1
        Add-UInt16 $Output 32
        Add-UInt32 $Output $Image.Bytes.Length
        Add-UInt32 $Output $Offset
        $Offset += $Image.Bytes.Length
    }

    foreach ($Image in $Images) {
        foreach ($Byte in $Image.Bytes) {
            $Output.Add($Byte)
        }
    }

    [System.IO.File]::WriteAllBytes($OutputPath, $Output.ToArray())
    Write-Host "Wrote $OutputPath"
}
finally {
    $Source.Dispose()
}
