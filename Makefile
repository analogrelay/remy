build:
	@cargo build --verbose

test:
	@cargo test --verbose

doc:
	@cargo doc --verbose

clean:
	@cargo clean --verbose

all: build test

rebuild: clean build

travis: all
