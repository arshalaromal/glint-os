use crate::drivers::vga::Writer;
use core::fmt::Write;
use core::sync::atomic::Ordering;
use crate::arch::time::TICKS;

pub fn execute(display: &mut Writer) {
    let ebx_val: u32;
    let edx_val: u32;
    let ecx_val: u32;

    unsafe {
        core::arch::asm!(
            "push rbx",          // Save LLVM's reserved rbx register to the stack
            "mov eax, 0",        //  Ask CPUID for the vendor string
            "cpuid",             
            "mov {0:e}, ebx",    // Copy ebx into a safe compiler-chosen register
            "pop rbx",           // Restore LLVM's rbx register so it doesn't crash
            out(reg) ebx_val,    //Tell LLVM to assign a safe register for {0:e}
            out("edx") edx_val,  
            out("ecx") ecx_val,  
            out("eax") _,       
        );
    }

    // Convert the 32-bit integers back into bytes (Little Endian format)
    let mut vendor = [0u8; 12];
    vendor[0..4].copy_from_slice(&ebx_val.to_le_bytes());
    vendor[4..8].copy_from_slice(&edx_val.to_le_bytes());
    vendor[8..12].copy_from_slice(&ecx_val.to_le_bytes());

    let vendor_str = core::str::from_utf8(&vendor).unwrap_or("Unknown");

    let total_seconds = TICKS.load(Ordering::Relaxed) / 1000;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;

    display.color = [0, 255, 255]; 
    let _ = writeln!(display, "GLINT OS System Information");
    
    display.color = [255, 255, 255]; 
    let _ = writeln!(display, "-----------------------------------------");
    let _ = writeln!(display, " OS          : Glint OS v0.1.0 (x86_64)");
    let _ = writeln!(display, " CPU Vendor  : {}", vendor_str); // Will likely say "GenuineIntel" on your i5
    let width = display.width;
    let height = display.height;
    let _ = writeln!(display, " Framebuffer : {}x{} px", width, height);
    let _ = writeln!(display, " Uptime      : {}m {}s", minutes, seconds);
    let _ = writeln!(display, "-----------------------------------------");
}