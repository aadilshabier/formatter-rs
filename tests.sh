
echo "Running debug mode"

cargo build -q
./target/debug/formatter-rs whitespace -d tmp/hugein.txt tmp/a.txt 
echo "Running release mode"

cargo build --release -q
./target/release/formatter-rs whitespace -d tmp/hugein.txt tmp/a.txt 