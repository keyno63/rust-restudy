@echo off
setlocal

set "VS_DEV_CMD=C:\BuildTools\Common7\Tools\VsDevCmd.bat"
set "CARGO_EXE=%USERPROFILE%\.cargo\bin\cargo.exe"

if not exist "%VS_DEV_CMD%" (
  echo VsDevCmd.bat not found: %VS_DEV_CMD%
  exit /b 1
)

if not exist "%CARGO_EXE%" (
  echo cargo.exe not found: %CARGO_EXE%
  exit /b 1
)

if "%~1"=="" (
  call "%VS_DEV_CMD%" -arch=x64 -host_arch=x64 >nul && "%CARGO_EXE%" run
) else (
  call "%VS_DEV_CMD%" -arch=x64 -host_arch=x64 >nul && "%CARGO_EXE%" %*
)

exit /b %errorlevel%
