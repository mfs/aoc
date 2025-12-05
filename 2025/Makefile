README.md:
	scripts/readme.sh

%:
	cargo run  --bin $@ < input/$@.txt

%-simple:
	cargo run --bin $* < simple.txt

.PHONY: README.md
