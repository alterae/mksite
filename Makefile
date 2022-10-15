.PHONY: all build clean clean-docs docs serve

all: docs

build:
	@cargo build

clean:
	@cargo clean

clean-docs:
	@cd docs && cargo run clean

docs:
	@cd docs && cargo run build

serve: clean-docs docs
	@pnpx serve docs/out
