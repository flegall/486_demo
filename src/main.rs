#![no_std]
#![no_main]

mod dos_tests;
use crate::dos_tests::file::file_read_test;
use _486_demo::*;

entry!(main);

fn main() {
    file_read_test();
}
