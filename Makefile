all:
	git checkout -f *.gir
	./fix.sh
	cd libphosh/sys && ../../gir/target/debug/gir -o .
	cd libphosh && ../gir/target/debug/gir -o .
	cargo build --examples
	cargo test
