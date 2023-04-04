CC=cargo
FMT=fmt

OPTIONS=

default: fmt
	$(CC) build --all-features
	@make example

fmt:
	$(CC) fmt --all

check:
	$(CC) test --all

example:
	$(CC) build --example foo_plugin
	$(CC) build --example macros_ex

clean:
	$(CC) clean
