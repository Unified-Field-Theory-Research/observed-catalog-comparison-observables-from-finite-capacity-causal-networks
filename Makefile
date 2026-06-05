.PHONY: test-fast rust-test lean-build check

test-fast:
	cargo test --workspace --test observed_catalog_comparison_observables_gate

rust-test:
	cargo test --workspace

lean-build:
	cd GPD/formal && lake build

check: test-fast lean-build
