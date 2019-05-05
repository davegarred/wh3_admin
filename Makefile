


build:
	cargo build --release --target x86_64-unknown-linux-musl
	cp ./target/release/bootstrap ./bootstrap 
	zip lambda.zip bootstrap 
	rm bootstrap
