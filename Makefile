.PHONY: remy rudy

remy:
	@cargo test --manifest-path ./remy/Cargo.toml --verbose

rudy:
	@cargo test --manifest-path ./rudy/Cargo.toml --verbose

build:
	@cargo build --manifest-path ./remy/Cargo.toml --verbose
	@cargo build --manifest-path ./rudy/Cargo.toml --verbose

doc:
	@cargo doc --manifest-path ./remy/Cargo.toml --verbose

clean:
	@cargo clean --manifest-path ./remy/Cargo.toml --verbose
	@cargo clean --manifest-path ./rudy/Cargo.toml --verbose

all: remy rudy

rebuild: clean all

travis: all doc
