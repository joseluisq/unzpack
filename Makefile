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
	@if [ ! -f "/tmp/static-web-server.zip" ]; then \
		echo "Downloading \"static-web-server.zip\" sample file..."; \
		curl -sLo /tmp/static-web-server.zip \
			https://github.com/joseluisq/static-web-server/archive/v1.9.0.zip; \
		sleep 3; \
	fi
	@cargo test --lib
.PHONY: test

fmt:
	@echo "Fixing and formatting source files..."
	@cargo fix
	@cargo fmt --all
.PHONY: fmt
