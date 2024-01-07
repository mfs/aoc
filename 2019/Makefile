README.md:
	echo "# Advent of Code 2023" > README.md
	echo >> README.md
	tokei --files --types rust --sort files --columns 80 src/bin/p??.rs | sed 's/^/    /' | sed 's/━/=/g' | sed 's/─/-/g' >> README.md

%:
	cargo run --release --bin $@ < input/$@.txt

%-debug:
	cargo run --bin $* < input/$*.txt

%-simple:
	cargo run --release --bin $* < simple.txt

.PHONY: README.md
