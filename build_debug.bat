@echo off

cargo build
rmdir /s /q exe 1>NUL 2>&1
mkdir exe 1>NUL 2>&1
move target\debug\brush.exe exe\brush_DEBUG.exe
cargo clean
pause