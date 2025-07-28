@echo off

cargo build --release
rmdir /s /q exe 1>NUL 2>&1
mkdir exe 1>NUL 2>&1
move target\release\brush.exe exe\brush.exe
cargo clean
pause