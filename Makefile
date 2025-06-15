.PHONY: build
build:
	cargo build
	cargo build --release

.PHONY: runtty
runtty: build
	./target/release/rusty-apple -t tty -w 100 -h 75

.PHONY: runtg
runtg: build
	./target/release/rusty-apple -t tg -w 15 -h 10

.PHONY: compose
compose:
	docker compose up --build -d

.PHONY: decompose
decompose:
	docker compose down
