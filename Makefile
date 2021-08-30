.PHONY: install
install:
	cargo install --force --path .

.PHONY: time
time:
	cargo +nightly build -Z timings