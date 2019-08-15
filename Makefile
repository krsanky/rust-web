all: deploy-web

target/release/rust_web:
	cargo build --release

deploy-web: target/release/rust_web
	cp target/release/rust_web /var/www/htdocs/index.cgi

clean:
	cargo clean

.PHONY: all clean deploy-cgi target/release/rust_web

