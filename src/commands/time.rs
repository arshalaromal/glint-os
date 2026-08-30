use crate::drivers::vga::Writer;
use core::fmt::Write;
use crate::drivers::rtc;

pub fn execute(display: &mut Writer) {
    let dt = rtc::read_datetime();
    
    display.color = [255, 255, 0]; 
    
 
    let _ = write!(
        display, 
        "Hardware Clock: 20{:02}-{:02}-{:02} {:02}:{:02}:{:02} UTC\n",
        dt.year, dt.month, dt.day, dt.hours, dt.minutes, dt.seconds
    );
}