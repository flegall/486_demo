#![no_std]
#![no_main]


use _486_demo::*;
use crate::dos_tests::file::file_read_test;

entry!(main);

fn main() {
    println!("Hello, World!");
    file_read_test();
    file_read_test();
}
