all: deploy-web

target/release/rust_web:
	cargo build --release

deploy-web: target/release/rust_web
	#cp index /var/www/htdocs/index.cgi
	#cp -r tmpl /var/www/htdocs/
	#cp -r static /var/www/htdocs/
	cp target/release/rust_web /var/www/htdocs/index.cgi

clean:
	cargo clean

.PHONY: all clean deploy-cgi target/release/rust_web

