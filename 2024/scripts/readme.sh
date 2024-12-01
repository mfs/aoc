#!/bin/bash

cargo build --release

echo "# Advent of Code 2024" > README.md
echo >> README.md

echo "## Benchmarks" >> README.md
echo >> README.md
hyperfine -w 3 "target/release/p01 < input/p01.txt" --export-markdown hf.md
cat hf.md >> README.md
rm hf.md
echo >> README.md

echo "## Lines of Code" >> README.md
echo >> README.md
scc --by-file -i rs -f csv --exclude-file template.rs --exclude-file csv2md.rs | cargo run --release --bin csv2md >> README.md
echo >> README.md


