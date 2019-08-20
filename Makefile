all: deploy-web

BIN="target/release/webpage"

$(BIN):
	cargo build --release

deploy-web: $(BIN) 
	cp $(BIN) /var/www/htdocs/index.cgi

clean:
	cargo clean

test:
	RUSTDOC=/usr/local/bin/rustdoc cargo test
	#RUSTDOCFLAGS='--sysroot /usr/local' cargo test

.PHONY: all clean test deploy-cgi $(BIN)

