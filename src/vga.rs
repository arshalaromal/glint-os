use core::convert::Infallible;
use core::fmt;
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
            cursor_y: 25, 
        }
    }

    pub fn clear_screen(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let offset = (y * self.pitch) + (x * self.bytes_per_pixel);
                unsafe {
                    //background color is black
                    *self.base_ptr.add(offset) = 0;     // B
                    *self.base_ptr.add(offset + 1) = 0; // G
                    *self.base_ptr.add(offset + 2) = 0; // R
                }
            }
        }
        self.cursor_x = 10;
        self.cursor_y = 25;
    }

    fn check_bounds(&mut self) {
        if self.cursor_y > self.height as i32 - 10 {
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
                self.cursor_y += 22; // Profont 18 is ~22px tall
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
            
            // Draw the single character
            let _ = Text::new(char_str, Point::new(self.cursor_x, self.cursor_y), style).draw(self);
            
            self.cursor_x += 12; // Profont 18 is ~12px wide
        }
        Ok(())
    }
}