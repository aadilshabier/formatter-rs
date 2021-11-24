#/usr/bin/sh
echo "Running release mode"

cargo build --release -q
./target/release/formatter-rs whitespace -d tmp/hugein.txt tmp/a.txt 
