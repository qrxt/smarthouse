test:
	cargo fmt --check
	cargo clippy
	cargo test

watch:
	cargo watch -x run

#

start:
	cargo run --example client