#![no_std]
#![no_main]

mod vga;
mod gdt; 

use core::panic::PanicInfo;
use core::fmt::Write;
use limine::request::FramebufferRequest;

static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    if let Some(response) = FRAMEBUFFER_REQUEST.response() {
        if let Some(fb) = response.framebuffers().first() {
            
            let mut display = vga::Writer::new(
                fb.address() as *mut u8,
                fb.width as usize,
                fb.height as usize,
                fb.pitch as usize,
                fb.bpp as usize,
            );

            display.clear_screen();

           
            display.color = [0, 255, 255];
            let _ = write!(display, "GLINT OS v0.1.0\n");
            let _ = write!(display, "================================================\n\n");

            display.color = [0, 255, 0];
            let _ = write!(display, "[ OK ] Limine Bootloader Handshaked\n");
            let _ = write!(display, "[ OK ] Framebuffer Mapped at {:p}\n", fb.address() as *const u8);
            
            // Initialize Architecture
            gdt::init();
            let _ = write!(display, "[ OK ] Global Descriptor Table (GDT) Loaded\n");
            let _ = write!(display, "[ OK ] Task State Segment (TSS) Activated\n");

            display.color = [255, 200, 0]; 
            let _ = write!(display, "\n[WAIT] Interrupt Descriptor Table (IDT) pending...\n");
            let _ = write!(display, "[WAIT] PIC Remapping pending...\n");

            display.color = [255, 255, 255];
            let _ = write!(display, "\nhalted.\n");
        }
    }

    loop {
        // Keep CPU at 0%
        unsafe { core::arch::asm!("hlt", options(nomem, nostack, preserves_flags)); }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack, preserves_flags)); }
    }
}