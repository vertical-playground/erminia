CLIPPY ?= 'clippy --all-targets --all-features -- -D warnings'
FMT ?= fmt --all -- --check

.PHONY: watch
watch:
	cargo watch -x ${CLIPPY}

.PHONY: fmt
fmt:
	cargo ${FMT}

.PHONY: test
test:
	cargo test --all-features
