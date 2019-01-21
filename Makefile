release:
	cargo build --release
	cp target/release/gic bin/

install:
	cp bin/gic /usr/local/bin
	gic -V