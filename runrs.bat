:: script for running the rust solutions
@ECHO off

cargo build --release
"./target/release/aoc-2023.exe" %1