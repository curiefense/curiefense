TARGET=iptools.so

$(TARGET): src/lib.rs src/avltree.rs
	cargo build --release
	cp target/release/libiptools.so $@

test: $(TARGET)
	cargo fmt --all -- --check
	cargo test
	lua5.1 test.lua
	lua5.1 test_sigset.lua
	lua5.1 test_geoip.lua

.PHONY: test

