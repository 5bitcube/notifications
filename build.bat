@echo off
cd /d "%~dp0"
cargo build --release 2>&1
if %errorlevel%==0 (
    copy /Y target\release\notifications.exe . >nul
    rmdir /s /q target
    echo Built: notifications.exe
)
