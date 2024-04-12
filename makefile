default: build

build:
	cargo build
	cp target/debug/chipeight chipeight
clean:
	cargo clean
release: clean
	cargo build --release
	cp target/release/chipeight chipeight
dependencies:
	cargo tree
