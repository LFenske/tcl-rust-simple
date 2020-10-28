#!/usr/bin/tclsh

# location of the dll you compiled with Rust
load "hello/target/debug/libfoo.so" 

mycmd parameter another "and a third"
# no parameters
mycmd
