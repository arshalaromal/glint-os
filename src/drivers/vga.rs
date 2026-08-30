use core::convert::Infallible;
use core::fmt::{self, Write};
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{Rgb888, RgbColor},
    prelude::*,
    text::Text,
    mono_font::MonoTextStyle,
};
use profont::PROFONT_18_POINT;

pub struct Writer {
    base_ptr: *mut u8,
    pub width: usize,
    pub height: usize,
    pitch: usize,
    bytes_per_pixel: usize,
    pub color: [u8; 3],
    cursor_x: i32,
    cursor_y: i32,
}

impl Writer {
    pub fn new(base_ptr: *mut u8, width: usize, height: usize, pitch: usize, bpp: usize) -> Self {
        Self {
            base_ptr,
            width,
            height,
            pitch,
            bytes_per_pixel: bpp / 8,
            color: [255, 255, 255], 
            cursor_x: 10,
            cursor_y: 60, 
        }
    }

    /// Draws filled rectangles with 32-bit Alpha byte handling 
    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: [u8; 3]) {
        for row in y..(y + height) {
            for col in x..(x + width) {
                if row < self.height && col < self.width {
                    let offset = (row * self.pitch) + (col * self.bytes_per_pixel);
                    unsafe {
                        *self.base_ptr.add(offset) = color[2];     // B
                        *self.base_ptr.add(offset + 1) = color[1]; // G
                        *self.base_ptr.add(offset + 2) = color[0]; // R
                        
                        //aalpha byte to 255 (Opaque) for 32 bpp framebuffers
                        if self.bytes_per_pixel == 4 {
                            *self.base_ptr.add(offset + 3) = 0xFF; 
                        }
                    }
                }
            }
        }
    }

    /// Draws the top blue header bar and writes the title text
    pub fn draw_header(&mut self) {
        // Draw 35px high bright blue bar across top
        self.fill_rect(0, 0, self.width, 35, [0, 80, 180]);

        // Save active terminal state
        let saved_x = self.cursor_x;
        let saved_y = self.cursor_y;
        let saved_color = self.color;

        // Draw header string inside blue bar
        self.cursor_x = 10;
        self.cursor_y = 22;
        self.color = [255, 255, 255];
        let _ = write!(self, "Glint OS v0.1.0");

        // Restore terminal state
        self.cursor_x = saved_x;
        self.cursor_y = saved_y;
        self.color = saved_color;
    }

    pub fn clear_screen(&mut self) {
        let total_bytes = self.height * self.pitch;
        unsafe {
            core::ptr::write_bytes(self.base_ptr, 0, total_bytes);
        }
        
        self.cursor_x = 10;
        self.cursor_y = 60; // Keep shell input below header bar (35px)

        //Always redraw header bar + header text on screen clear
        self.draw_header();
    }

    pub fn set_cursor(&mut self, x: i32, y: i32) {
        self.cursor_x = x;
        self.cursor_y = y;
    }

    /// Erases character under cursor using expanded bounding box 
    pub fn backspace(&mut self) {
        if self.cursor_x > 34 { 
            self.cursor_x -= 12; // Shift back one character width
            
            self.fill_rect(
                self.cursor_x as usize, 
                (self.cursor_y - 20) as usize, 
                14, 26, [0, 0, 0]
            );
        }
    }

    fn check_bounds(&mut self) {
        if self.cursor_y > self.height as i32 - 30 {
            self.clear_screen();
        }
    }
}

impl OriginDimensions for Writer {
    fn size(&self) -> Size { Size::new(self.width as u32, self.height as u32) }
}

impl DrawTarget for Writer {
    type Color = Rgb888;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where I: IntoIterator<Item = Pixel<Self::Color>> {
        for Pixel(coord, color) in pixels.into_iter() {
            if coord.x >= 0 && coord.x < self.width as i32 && coord.y >= 0 && coord.y < self.height as i32 {
                let offset = (coord.y as usize * self.pitch) + (coord.x as usize * self.bytes_per_pixel);
                unsafe {
                    *self.base_ptr.add(offset) = color.b();
                    *self.base_ptr.add(offset + 1) = color.g();
                    *self.base_ptr.add(offset + 2) = color.r();
                    
                    if self.bytes_per_pixel == 4 {
                        *self.base_ptr.add(offset + 3) = 0xFF;
                    }
                }
            }
        }
        Ok(())
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let style = MonoTextStyle::new(
            &PROFONT_18_POINT, 
            Rgb888::new(self.color[0], self.color[1], self.color[2])
        );

        for c in s.chars() {
            if c == '\n' {
                self.cursor_x = 10;
                self.cursor_y += 22;
                self.check_bounds();
                continue;
            }

            if self.cursor_x > self.width as i32 - 12 {
                self.cursor_x = 10;
                self.cursor_y += 22;
                self.check_bounds();
            }

            let mut buf = [0; 4];
            let char_str = c.encode_utf8(&mut buf);
            
            let _ = Text::new(char_str, Point::new(self.cursor_x, self.cursor_y), style).draw(self);
            
            self.cursor_x += 12;
        }
        Ok(())
    }
}