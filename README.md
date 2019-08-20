cargo not working on 32bit OpenBSD

This makes "cargo test" not show an error:
RUSTDOC=/usr/local/bin/rustdoc cargo test
or?:
% RUSTDOCFLAGS='--sysroot /usr/local' cargo test



