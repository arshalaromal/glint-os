use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyle, Triangle},
    text::Text,
    mono_font::MonoTextStyle,
};
use profont::PROFONT_24_POINT;
use crate::drivers::vga::Writer;
use crate::arch::time::sleep_ms;

pub fn run(display: &mut Writer) {
    //Clear background to black
    display.clear(Rgb888::new(0, 0, 0)).unwrap();

    let center_x = display.width as i32 / 2;
    let center_y = display.height as i32 / 2;

    //Draw Glint OS text slightly offset to the right
    let text_style = MonoTextStyle::new(&PROFONT_24_POINT, Rgb888::new(255, 255, 255));
    Text::new("Glint OS", Point::new(center_x - 30, center_y + 8), text_style)
        .draw(display)
        .unwrap();

    // Define the center coordinates for the Glint logo (to the left of the text)
    let logo_x = center_x - 80;
    let logo_y = center_y;

    // Helper closure to redraw the 4-pointed star in a specific color
    let mut draw_star = |color: Rgb888| {
        let h = 30; // Length of the star points
        let w = 5;  // Width at the center base
        let style = PrimitiveStyle::with_fill(color);

        // Top point
        Triangle::new(Point::new(logo_x - w, logo_y), Point::new(logo_x + w, logo_y), Point::new(logo_x, logo_y - h))
            .into_styled(style).draw(display).unwrap();
        // Bottom point
        Triangle::new(Point::new(logo_x - w, logo_y), Point::new(logo_x + w, logo_y), Point::new(logo_x, logo_y + h))
            .into_styled(style).draw(display).unwrap();
        // Left point
        Triangle::new(Point::new(logo_x, logo_y - w), Point::new(logo_x, logo_y + w), Point::new(logo_x - h, logo_y))
            .into_styled(style).draw(display).unwrap();
        // Right point
        Triangle::new(Point::new(logo_x, logo_y - w), Point::new(logo_x, logo_y + w), Point::new(logo_x + h, logo_y))
            .into_styled(style).draw(display).unwrap();
    };

    //Red to Blue
    for i in 0..=50 {
        let r = 255 - (i * 255 / 50) as u8;
        let b = (i * 255 / 50) as u8;
        draw_star(Rgb888::new(r, 0, b));
        sleep_ms(15);
    }

    //Blue to Green
    for i in 0..=50 {
        let b = 255 - (i * 255 / 50) as u8;
        let g = (i * 255 / 50) as u8;
        draw_star(Rgb888::new(0, g, b));
        sleep_ms(15);
    }

    //Green to White
    for i in 0..=50 {
        let r = (i * 255 / 50) as u8;
        let b = (i * 255 / 50) as u8;
        draw_star(Rgb888::new(r, 255, b)); 
        sleep_ms(15);
    }

  
    sleep_ms(1500);
}