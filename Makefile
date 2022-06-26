test:
	cargo fmt --check
	cargo clippy
	cargo test

watch:
	cargo watch -x run

homework-examples:
	cargo run --example rooms
	cargo run --example devices