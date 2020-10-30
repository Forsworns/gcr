c = cargo

.PHONY: build
build:
	$(c) build --release

.PHONY: build-dev
build-dev:
	$(c) build -v

.PHONY: test
test:
	$(c) test -j 1 -v

.PHONY: fmt
fmt:
	$(c) fmt

.PHONY: publish
publish:
	$(c) publish -v

.PHONY: commit
commit: fmt
	grc -a .