.PHONY: test-018 test-019 test-all build-018 build-019 clippy

test-018:
	cargo test --manifest-path tests/test_018/Cargo.toml

test-019:
	cargo test --manifest-path tests/test_019/Cargo.toml

test-all: test-018 test-019

build-018:
	cargo build --no-default-features --features 0.18

build-019:
	cargo build --no-default-features --features 0.19

build-all: build-018 build-019

clippy:
	cargo clippy --no-default-features --features 0.18 -- -D warnings
