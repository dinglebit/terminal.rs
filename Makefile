.EXPORT_ALL_VARIABLES:

CARGO_HOME ?= $(HOME)/.rustup

test:
	cargo test

build:
	cargo build

cover:
	cargo kcov --lib

cover-open: 
	cargo kcov --lib --open
