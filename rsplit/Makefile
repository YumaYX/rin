default: clean data build run

build:
	cargo build --release

run:
	time ./target/release/rsplit -f num.txt -l 3

data:
	seq 10 > num.txt

clean:
	rm -rf rsplit target num.txt
