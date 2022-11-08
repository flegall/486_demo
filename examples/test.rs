#![no_std]
#![no_main]

use crate::dos_tests::file::file_read_test;
use _486_demo::*;

entry!(main);

fn main() {
    println!("Hello, World!");
    file_read_test();
    file_read_test();
}
