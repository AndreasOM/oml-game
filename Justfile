fmt:
	cargo +nightly fmt

clippy:
        RUSTFLAGS=-Awarnings cargo clippy -- -Aclippy::all -Aclippy::erasing-op -Fclippy::correctness -Aclippy::erasing-op


