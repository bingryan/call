.PHONY: install
install:
	cargo install --force --path .

.PHONY: tarpaulin
tarpaulin:
	cargo tarpaulin -v

.PHONY: time
time:
	cargo +nightly build -Z timings

.PHONY: commit
commit:
	pre-commit run --all-files
