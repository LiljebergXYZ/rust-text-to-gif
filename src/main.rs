extern crate sdl2;

use sdl2::pixels::{PixelFormatEnum};
use std::fs::File;
use gif::{Encoder, Repeat, SetParameter, Frame};
use sdl2::rect::Rect;
use std::fs;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub fn main() {
    let size = 64;

    let text = fs::read_to_string("./input.txt").expect("Unable to read input");

    // Setup ttf
    let ttf_context = sdl2::ttf::init().expect("Unable to initialize ttf");
    let font = ttf_context.load_font("./Consolas.ttf", 32)
        .expect("Unable to load font");
    let font_color = sdl2::pixels::Color::from((0, 255, 0));
    let text_surface = font.render(text.as_str()).solid(font_color)
        .expect("Unable to make text surface");

    let mut surface = sdl2::surface::Surface::new(size as u32, size as u32, PixelFormatEnum::RGB24)
        .expect("Unable to create surface");

    // Setup gif encoder
    let mut image = File::create("output.gif").expect("Unable to create file");
    let mut encoder = Encoder::new(&mut image, size, size, &[])
        .expect("Unable to create encoder");
    encoder.set(Repeat::Infinite).expect("Unable to set repeat");

    let center_y = size / 2 - (text_surface.as_ref().height() / 2) as u16;
    let text_width = (text_surface.as_ref().width()) as i32 * -1;

    let scroll_width = (size / 6) as i32;
    let mut scroll_x = scroll_width + size as i32;
    while scroll_x > text_width - scroll_width {
        surface.fill_rect(None, sdl2::pixels::Color::from((0, 0, 0))).expect("Unable to fill rect");
        text_surface.blit(None, surface.as_mut(), rect!(scroll_x, center_y, 0, 0)).expect("Unable to blit");
        surface.with_lock(|s| {
            let frame = Frame::from_rgb(size, size, s);
            encoder.write_frame(&frame).expect("Unable to add frame to encoder");
        });

        scroll_x -= scroll_width;
    }
}