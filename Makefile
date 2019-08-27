all: deploy-web

WP="target/release/webpage"
MN="target/release/main"

$(WP):
	cargo build --release
$(MN):
	cargo build --release

deploy-web: $(WP) $(MN)
	cp $(WP) /var/www/htdocs/index.cgi
	cp $(MN) /var/www/htdocs/main.cgi
	cp -r static /var/www/htdocs/

clean:
	cargo clean

test:
	RUSTDOC=/usr/local/bin/rustdoc cargo test
	#RUSTDOCFLAGS='--sysroot /usr/local' cargo test

.PHONY: all clean test deploy-cgi 

