.PHONY: build
build:
	cargo build
	cargo build --release

.PHONY: run
run: build
	./target/release/rusty-apple

.PHONY: compose
compose:
	docker compose up --build -d

.PHONY: decompose
decompose:
	docker compose down
