default: debug-run

debug-run:
	cargo run

clean:
	cargo clean

release-linux: clean
	cargo build --release --target x86_64-unknown-linux-gnu

release-windows: clean
	PKG_CONFIG_SYSROOT_DIR=/usr/x86_64-w64-mingw32/ cargo xwin build --release --target x86_64-pc-windows-msvc

release-all: release-linux release-windows