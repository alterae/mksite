.PHONY: all docs serve gh-pages

all: docs

docs:
	cd docs && cargo r clean && cargo r build

serve: docs
	pnpx serve docs/out

gh-pages:
	cd docs && cargo r build
