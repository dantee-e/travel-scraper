run:
	cargo build
	gcc rust_interface.c -o rust_interface -ltravel_scraper -L./target/release/
	LD_LIBRARY_PATH=./target/debug/ ./rust_interface

release:
	cargo build --release
	LD_LIBRARY_PATH=./target/release/ ./rust_interface
	gcc rust_interface.c -o rust_interface -ltravel_scraper -L./target/release/