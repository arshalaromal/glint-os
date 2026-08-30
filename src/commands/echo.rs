use crate::drivers::vga::Writer;
use core::fmt::Write;

pub fn execute(display: &mut Writer, args: &str) {
    display.color = [255, 255, 255];
    let _ = write!(display, "{}\n", args);
}