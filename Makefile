test:
	cargo fmt --check
	cargo clippy
	cargo test

watch:
	cargo watch -x run

#

run_server:
	cargo run --example server  

run_client:
	cargo run --example client  