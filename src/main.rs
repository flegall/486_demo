#![no_std]
#![no_main]


mod dos_tests;
use _486_demo::*;
use crate::dos_tests::file::file_read_test;

entry!(main);

fn main() {
    file_read_test();
}
