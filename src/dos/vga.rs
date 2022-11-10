use core::arch::asm;

pub fn set_video_mode(mode: u8) {
    unsafe {
        asm!(
            "int 10h",
            inout("ax") mode as u16 => _,
        );
    }
}

pub fn slow_plot_pixel(x: u16, y: u16, color: u8) {
    unsafe {
        asm!(
            "int 10h",
            in("ax") (0x0C00u16) | (color as u16),
            in("bh") 0u8,
            in("cx") x,
            in("dx") y,
        );
    }
}
