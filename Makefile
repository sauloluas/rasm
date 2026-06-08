.PHONY: run less

run:
	cargo run prototype.rasm


less:
	cargo run prototype.rasm 2>&1 | less -R
