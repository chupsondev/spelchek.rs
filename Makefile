all: 
	mkdir -p $$HOME/.spelchek/
	cp ./dict.txt $$HOME/.spelchek/
	mkdir -p $$HOME/.local/bin
	cargo build --release
	cp ./target/release/spelchek $$HOME/.local/bin

dict: 
	mkdir -p $$HOME/.spelchek/
	cp ./dict.txt $$HOME/.spelchek/dict.txt
