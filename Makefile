.PHONY: test run

run:
	cargo run

test:
	cargo test
	cd feedbin-api-client && cargo test
