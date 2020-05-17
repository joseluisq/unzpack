run:
	@echo "Running example application..."
	@rustc -vV
	@cd examples/ && cargo run
.PHONY: run

build:
	@echo "Building example application..."
	@rustc -vV
	@cargo build --release \
		--manifest-path=examples/Cargo.toml \
		--target x86_64-unknown-linux-musl
.PHONY: build

test:
	@echo "Testing library..."
	@rustc -vV
	@curl -sLo /tmp/static-web-server.zip \
		https://github.com/joseluisq/static-web-server/archive/v1.9.0.zip
	@cargo test
.PHONY: test

fmt:
	@echo "Fixing and formatting source files..."
	@cargo fix
	@cargo fmt --all
.PHONY: fmt
