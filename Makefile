build:
	cargo build --release --bin auth-token

build-all: build-darwin-aarch64 build-darwin-x86_64 build-windows-x86_64 build-linux-x86_64-glibc build-linux-x86_64-musl

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

build-darwin-aarch64:
	cargo build --release --target aarch64-apple-darwin --bin auth-token 

build-darwin-x86_64:
	cargo build --release --target x86_64-apple-darwin --bin auth-token

build-windows-x86_64:
	cargo build --release --target x86_64-pc-windows-gnu  --bin auth-token

build-linux-x86_64-glibc:
	cargo build --release --target x86_64-unknown-linux-gnu  --bin auth-token

build-linux-x86_64-musl:
	cargo build --release --target x86_64-unknown-linux-musl  --bin auth-token

format:
	cargo fmt
