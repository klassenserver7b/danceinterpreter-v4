default: debug

debug:
	cargo build

release:
	cargo build --release --target x86_64-unknown-linux-gnu

clean:
	cargo clean


windows:
	PKG_CONFIG_SYSROOT_DIR=/usr/x86_64-w64-mingw32/ cargo xwin build --release --target x86_64-pc-windows-msvc

release-all: release windows