test-all: format lint test 

run-all: test-all audit

format:
    rustup component add rustfmt
    cargo fmt -- --check

lint:
    rustup component add clippy
    cargo clippy -- -D warnings

test:
    cargo test --workspace --verbose

audit:
    cargo install cargo-audit
    cargo audit