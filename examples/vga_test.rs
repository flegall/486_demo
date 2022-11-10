#![no_std]
#![no_main]

use _486_demo::*;

entry!(main);

fn main() {
    println!("Hello, VGA!");
    dos::vga::set_video_mode(0x13);

    const COLOR: u8 = 95;

    dos::vga::slow_plot_pixel(100, 100, COLOR);
    dos::vga::slow_plot_pixel(101, 100, COLOR);
    dos::vga::slow_plot_pixel(102, 100, COLOR);
    dos::vga::slow_plot_pixel(103, 100, COLOR);
    dos::vga::slow_plot_pixel(100, 101, COLOR);
    dos::vga::slow_plot_pixel(101, 101, COLOR);
    dos::vga::slow_plot_pixel(102, 101, COLOR);
    dos::vga::slow_plot_pixel(103, 101, COLOR);
    dos::vga::slow_plot_pixel(100, 102, COLOR);
    dos::vga::slow_plot_pixel(101, 102, COLOR);
    dos::vga::slow_plot_pixel(102, 102, COLOR);
    dos::vga::slow_plot_pixel(103, 102, COLOR);
    dos::vga::slow_plot_pixel(100, 103, COLOR);
    dos::vga::slow_plot_pixel(101, 103, COLOR);
    dos::vga::slow_plot_pixel(102, 103, COLOR);
    dos::vga::slow_plot_pixel(103, 103, COLOR);

    // dos::vga::set_video_mode(0x03);
}
