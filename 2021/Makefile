README.md:
	echo "# Advent of Code 2021" > README.md
	echo >> README.md
	tokei --files --type rust --sort files --columns 80 src/bin/p??.rs | sed 's/^/    /' >> README.md

.PHONY: README.md
