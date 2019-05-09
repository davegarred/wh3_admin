
clean:
	cargo clean

build:
	cargo build --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/bootstrap ./bootstrap
	zip lambda.zip bootstrap 
	rm bootstrap

deploy: build
	aws lambda update-function-code \
    		--function-name arn:aws:lambda:us-west-2:202214144554:function:wh3_admin \
    		--zip-file fileb://lambda.zip
