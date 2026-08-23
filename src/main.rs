#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;
use limine::request::FramebufferRequest;

use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::Rgb888,
    prelude::*,
    text::Text,
};
use profont::PROFONT_18_POINT;

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

            // Clear the screen
            display.clear(Rgb888::new(15, 18, 24)).unwrap();

            // Define font styles with distinct colors
            let style_title  = MonoTextStyle::new(&PROFONT_18_POINT, Rgb888::new(0, 229, 255)); // Bright Cyan
            let style_sub    = MonoTextStyle::new(&PROFONT_18_POINT, Rgb888::new(140, 150, 170)); // Slate Grey
            let style_ok     = MonoTextStyle::new(&PROFONT_18_POINT, Rgb888::new(0, 230, 118)); // Neon Green
            let style_warn   = MonoTextStyle::new(&PROFONT_18_POINT, Rgb888::new(255, 171, 0));   // Amber Gold
            let style_body   = MonoTextStyle::new(&PROFONT_18_POINT, Rgb888::new(240, 244, 248)); // Crisp White

            // Draw Header Banner
            Text::new("GLINT KERNEL v0.1.0", Point::new(30, 45), style_title)
                .draw(&mut display)
                .unwrap();
        }
    }

    loop {
        halt_cpu();
    }
}
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        halt_cpu();
    }
}

pub fn halt_cpu() {
    unsafe {
        core::arch::asm!("hlt", options(nomem, nostack, preserves_flags));
    }
}