.PHONY: test

test:
	cargo test
	cd feedbin-api-client && cargo test
