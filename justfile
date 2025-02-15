@_:
	just --list

setup:
	chmod +x .githooks/*
	git config --local core.hooksPath .githooks

run *ARGS:
	cargo run --bin astre -- {{ARGS}}

doc *ARGS:
	RUSTFLAGS="-A unused_variables -A dead_code" RUST_BACKTRACE=1 cargo doc --no-deps {{ARGS}}

book *ARGS:
	mdbook {{ARGS}}

snapshot-cleanup *ARGS:
	RUSTFLAGS="-A unused_variables -A dead_code" RUST_BACKTRACE=1 cargo insta test --accept --unreferenced delete {{ARGS}}

snapshot *ARGS:
	RUSTFLAGS="-A unused_variables -A dead_code" RUST_BACKTRACE=1 cargo insta test --accept {{ARGS}}

expand *ARGS:
	RUSTFLAGS="-A unused_variables -A dead_code" RUST_BACKTRACE=1 cargo expand {{ARGS}}

test *ARGS:
	RUSTFLAGS="-A unused_variables -A dead_code" RUST_BACKTRACE=1 cargo insta test {{ARGS}}

build *ARGS:
	RUSTFLAGS="-A unused_variables -A dead_code" RUST_BACKTRACE=1 cargo build {{ARGS}}