use crate::drivers::vga::Writer;

pub fn execute(display: &mut Writer) {
    display.clear_screen();
}