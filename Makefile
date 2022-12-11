all:
	make build
build:
	cargo build --release
setup:
	cd ./bin/ && ./setup
