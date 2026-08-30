use core::sync::atomic::{AtomicUsize, Ordering};
use x86_64::instructions::port::Port;

// A global counter that goes up every millisecond
pub static TICKS: AtomicUsize = AtomicUsize::new(0);

pub fn init_pit() {
    let mut cmd_port = Port::<u8>::new(0x43);
    let mut data_port = Port::<u8>::new(0x40);

    // Hardware clock is 1.193182 MHz. Divide by 1193 to get ~1000 Hz.
    let divider: u16 = 1193;

    unsafe {
        cmd_port.write(0x36);
        data_port.write((divider & 0xFF) as u8);
        data_port.write((divider >> 8) as u8);
    }
}

pub fn sleep_ms(ms: usize) {
    let start = TICKS.load(Ordering::Relaxed);
    while TICKS.load(Ordering::Relaxed) - start < ms {
        // Sleep the CPU until the next interrupt wakes it up.
        // This drops CPU usage to 0% while waiting.
        unsafe { core::arch::asm!("hlt", options(nomem, nostack, preserves_flags)); }
    }
}