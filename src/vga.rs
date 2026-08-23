use core::convert::Infallible;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{Rgb888, RgbColor}, 
    Pixel,
};

pub struct Writer {
    base_ptr: *mut u8,
    width: usize,
    height: usize,
    pitch: usize,
    bytes_per_pixel: usize,
}

impl Writer {
    pub fn new(base_ptr: *mut u8, width: usize, height: usize, pitch: usize, bpp: usize) -> Self {
        Self {
            base_ptr,
            width,
            height,
            pitch,
            bytes_per_pixel: bpp / 8,
        }
    }
}

impl OriginDimensions for Writer {
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}

impl DrawTarget for Writer {
    type Color = Rgb888;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            if coord.x >= 0 && coord.x < self.width as i32 && coord.y >= 0 && coord.y < self.height as i32 {
                let pixel_offset = (coord.y as usize * self.pitch) + (coord.x as usize * self.bytes_per_pixel);
                
                unsafe {
                    *self.base_ptr.add(pixel_offset) = color.b();
                    *self.base_ptr.add(pixel_offset + 1) = color.g();
                    *self.base_ptr.add(pixel_offset + 2) = color.r();
                }
            }
        }
        Ok(())
    }
}