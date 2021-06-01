build:
	cargo build --release  --bin tako

install-toolchains:
	rustup target add aarch64-apple-darwin
	rustup target add x86_64-apple-darwin
	rustup target add x86_64-pc-windows-gnu
	rustup target add x86_64-unknown-linux-gnu
	rustup target add x86_64-unknown-linux-musl

build-mac-aarch64:
	cargo build --release --target aarch64-apple-darwin --bin tako

build-mac:
	cargo build --release --target x86_64-apple-darwin --bin tako

build-win:
	cargo build --release --target x86_64-pc-windows-gnu  --bin tako

build-linux:
	cargo build --release --target x86_64-unknown-linux-gnu  --bin tako

build-linux-musl:
	cargo build --release --target x86_64-unknown-linux-musl  --bin tako

