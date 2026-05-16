.PHONY: help fmt check lint test test-minimal build doc examples audit deny sbom publish-dry-run facade-dry-run release-readiness facade-readiness verify

CRATE := use-signal
FOCUSED_CRATES := use-sample use-amplitude use-frequency use-signal-window use-signal-normalize use-signal-energy use-zero-crossing use-simple-filter

help:
	@printf "%s\n" \
		"help               Show available repository tasks" \
		"fmt                Check formatting with rustfmt" \
		"check              Run cargo check for the workspace" \
		"lint               Run clippy with warnings denied" \
		"test               Run workspace tests with all features" \
		"test-minimal       Run workspace tests with no default features" \
		"build              Build the workspace with all features" \
		"doc                Build workspace docs without dependencies" \
		"examples           Check all examples" \
		"audit              Run cargo-audit" \
		"deny               Run cargo-deny" \
		"sbom               Generate a CycloneDX SBOM" \
		"publish-dry-run    Dry-run publish each focused crate" \
		"facade-dry-run     Dry-run publish the umbrella crate" \
		"release-readiness  Run focused-crate pre-release checks" \
		"facade-readiness   Run umbrella pre-release checks" \
		"verify             Run the main workspace validation path"

fmt:
	cargo fmt --all -- --check

check:
	cargo check --workspace --all-features

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-features

test-minimal:
	cargo test --workspace --no-default-features

build:
	cargo build --workspace --all-features

doc:
	cargo doc --workspace --all-features --no-deps

examples:
	cargo check --workspace --all-features --examples

audit:
	cargo audit

deny:
	cargo deny check

sbom:
	cargo cyclonedx --manifest-path crates/$(CRATE)/Cargo.toml --all-features --format json --spec-version 1.5 --override-filename sbom.cyclonedx

publish-dry-run:
	@for crate in $(FOCUSED_CRATES); do \
		cargo package --list -p $$crate; \
		cargo publish --dry-run --allow-dirty -p $$crate; \
	done

facade-dry-run:
	cargo package --list -p $(CRATE)
	cargo publish --dry-run --allow-dirty -p $(CRATE)

release-readiness: verify examples test-minimal publish-dry-run

facade-readiness: verify examples test-minimal facade-dry-run

verify: fmt lint test build