.PHONY: build
build:
	cargo build
	cargo build --release

.PHONY: run
run: build
	./target/release/rusty-apple
