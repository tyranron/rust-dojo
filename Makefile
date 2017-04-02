# Format all project Rust sources with Rustfmt.
#
# More information about Rustfmt modes:
# https://github.com/rust-lang-nursery/rustfmt#running
#
# Usage:
#	make fmt [mode=...]

mode ?= overwrite

fmt:
	rustup run nightly cargo fmt -- --write-mode=$(mode)



.PHONY: fmt
