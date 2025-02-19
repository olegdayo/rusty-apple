.PHONY: build
build:
	cargo build
	cargo build --release

.PHONY: runtty
runtty: build
	./target/release/rusty-apple -t tty -w 60 -h 40

.PHONY: runtg
runtg: build
	./target/release/rusty-apple -t tg -w 20 -h 15

.PHONY: compose
compose:
	docker compose up --build -d

.PHONY: decompose
decompose:
	docker compose down
