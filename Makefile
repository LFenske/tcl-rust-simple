default:	run
.PHONY:	default

.PHONY:	run
run:	hello/target/debug/libfoo.so
	./foo.tcl

hello/target/debug/libfoo.so:	hello/src/lib.rs
	cd hello; cargo build

.PHONY:	clean
clean:
	cd hello; cargo clean; rm -f Cargo.lock
	rm -f *~ hello/src/*~
