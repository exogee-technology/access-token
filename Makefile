build:
	cargo build --release

install-toolchains:
	rustup target add aarch64-apple-darwin
	rustup target add x86_64-apple-darwin
	rustup target add x86_64-pc-windows-gnu
	rustup target add x86_64-unknown-linux-gnu
	rustup target add x86_64-unknown-linux-musl

build-mac-aarch64:
	cargo build --release --target aarch64-apple-darwin

build-mac:
	cargo build --release --target x86_64-apple-darwin

build-win:
	cargo build --release --target x86_64-pc-windows-gnu

build-linux:
	cargo build --release --target x86_64-unknown-linux-gnu

build-linux-musl:
	cargo build --release --target x86_64-unknown-linux-musl

