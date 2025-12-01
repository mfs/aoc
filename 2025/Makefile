README.md:
	scripts/readme.sh

%:
	cargo run --release --bin $@ < input/$@.txt

%-debug:
	cargo run --bin $* < input/$*.txt

%-simple:
	cargo run --release --bin $* < simple.txt

.PHONY: README.md
