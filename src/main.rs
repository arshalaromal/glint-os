#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod arch {
    pub mod gdt;
    pub mod interrupts;
    pub mod time;
}

pub mod drivers {
    pub mod vga;
    pub mod rtc;
    pub mod speaker;
}

pub mod splash;
pub mod shell; 
pub mod commands;

use core::panic::PanicInfo;
use core::fmt::Write;
use core::sync::atomic::Ordering;
use limine::request::FramebufferRequest;

static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    if let Some(response) = FRAMEBUFFER_REQUEST.response() {
        if let Some(fb) = response.framebuffers().first() {
            
            let mut display = drivers::vga::Writer::new(
                fb.address() as *mut u8, fb.width as usize,
                fb.height as usize, fb.pitch as usize, fb.bpp as usize,
            );
        arch::gdt::init();
        arch::interrupts::init();
        arch::time::init_pit();

        x86_64::instructions::interrupts::enable();
        
        crate::splash::run(&mut display);
        
        display.clear_screen();

        display.color = [0, 255, 0]; 
        let _ = write!(display, "> ");
        display.color = [255, 255, 255];

            // Boot the Shell
            let mut shell = shell::Shell::new();

            //0% CPU Event Loop
            loop {
                // Check if the keyboard interrupt
                let key = arch::interrupts::LAST_KEY.swap(0, Ordering::Relaxed);
                
                if key != 0 {
                    shell.handle_scancode(key, &mut display);
                }

                // Sleep the CPU until the next Timer tick or Keyboard press
                unsafe { core::arch::asm!("hlt", options(nomem, nostack, preserves_flags)); }
            }
        }
    }

    // Fallback if Limine fails to provide framebuffer
    loop { 
        unsafe { core::arch::asm!("hlt", options(nomem, nostack, preserves_flags)); } 
    }
}

//Debugging & Auto-Reboot Panic Handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Write crash log to QEMU's background terminal via the Serial Port (0x3F8)
    struct SerialPort;
    impl core::fmt::Write for SerialPort {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            let mut port = x86_64::instructions::port::Port::<u8>::new(0x3F8);
            for byte in s.bytes() { unsafe { port.write(byte); } }
            Ok(())
        }
    }
    
    let _ = writeln!(SerialPort, "\n\n*** GLINT OS KERNEL PANIC ***\n{}\n", info);
    
    // Pulse the PS/2 reset line to physically reboot the computer
    unsafe { x86_64::instructions::port::Port::<u8>::new(0x64).write(0xFE); }
    
    loop { unsafe { core::arch::asm!("hlt"); } }
}