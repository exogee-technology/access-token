build:
	cargo build --release --bin auth-token

build-gui:
	cargo build --release --bin auth-token-gui

publish:
	cargo publish

install-toolchains:
	rustup target add aarch64-apple-darwin
	rustup target add x86_64-apple-darwin
	rustup target add x86_64-pc-windows-gnu
	rustup target add x86_64-unknown-linux-gnu
	rustup target add x86_64-unknown-linux-musl

build-mac-aarch64:
	cargo build --release --target aarch64-apple-darwin --bin auth-token 

build-mac:
	cargo build --release --target x86_64-apple-darwin --bin auth-token

build-win:
	cargo build --release --target x86_64-pc-windows-gnu  --bin auth-token

build-linux:
	cargo build --release --target x86_64-unknown-linux-gnu  --bin auth-token

build-linux-musl:
	cargo build --release --target x86_64-unknown-linux-musl  --bin auth-token

format:
	cargo fmt
