#!/bin/bash

mkdir -pv target/build

cargo build --release --target x86_64-unknown-linux-gnu
cross build --release --target x86_64-pc-windows-gnu

cp target/x86_64-unknown-linux-gnu/release/gs target/build
cp target/x86_64-pc-windows-gnu/release/gs.exe target/build
