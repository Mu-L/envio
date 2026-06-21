.PHONY: all build run format lint install uninstall clean

BIN_NAME = envio

CARGO_FLAGS ?=

all: build

build:
	cargo build $(CARGO_FLAGS)

# make run ARGS="--help"
run:
	cargo run $(CARGO_FLAGS) --bin $(BIN_NAME) -- $(ARGS)

format:
	cargo fmt

lint:
	cargo clippy $(CARGO_FLAGS) --all-targets

install:
	cargo install --path . $(CARGO_FLAGS)

uninstall:
	cargo uninstall $(BIN_NAME)

clean:
	cargo clean
