param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]] $CargoArgs
)

$vsDevCmd = "C:\BuildTools\Common7\Tools\VsDevCmd.bat"
$cargoExe = Join-Path $HOME ".cargo\bin\cargo.exe"

if (-not (Test-Path $vsDevCmd)) {
    Write-Error "VsDevCmd.bat が見つかりません: $vsDevCmd"
    exit 1
}

if (-not (Test-Path $cargoExe)) {
    Write-Error "cargo.exe が見つかりません: $cargoExe"
    exit 1
}

if (-not $CargoArgs -or $CargoArgs.Count -eq 0) {
    $CargoArgs = @("run")
}

$escapedArgs = $CargoArgs | ForEach-Object { '"' + ($_ -replace '"', '\"') + '"' }
$cargoArgLine = $escapedArgs -join " "
$cmdLine = "`"$vsDevCmd`" -arch=x64 -host_arch=x64 >nul && `"$cargoExe`" $cargoArgLine"

cmd /c $cmdLine
exit $LASTEXITCODE
