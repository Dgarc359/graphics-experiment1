extern crate sdl2;

mod assets;
mod game;
mod local_util;
use local_util::linear2srgb;
use std::path::Path;
use std::env;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Rect};
use sdl2::image::{InitFlag, LoadTexture};
use std::time::Duration;
const WIDTH: usize = 255;
const HEIGHT: usize = 255;
const PITCH: usize = 4 * WIDTH;
const RESOLUTION: usize = WIDTH * HEIGHT * 4;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _img_ctx = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);

    let window = video_subsystem
        .window("rust-sdl2 demo", 1000, 1000)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump().unwrap();


    let pwd = env::current_dir().unwrap();
    let pwd_str = pwd.to_str().unwrap();
    let path = Path::new(pwd_str).join("src").join("smile.png");
    println!("{:?}", path.to_str().unwrap());
    let texture = texture_creator.load_texture(path).unwrap();
    let rect = Rect::new(10, 10, 50, 50);
    canvas.copy(&texture, None, rect).unwrap();
    canvas.present();


    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn make_circle(
    radius: f32,
    pixels_as_u8: &mut [u8],
) {
    let center_x = WIDTH / 2;
    let center_y = HEIGHT / 2;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let x_dist = if center_x < x {
                (x.checked_sub(center_x).unwrap()) as f32
            } else {
                (center_x.checked_sub(x).unwrap()) as f32
            };

            let y_dist = if center_y < y {
                (y.checked_sub(center_y).unwrap()) as f32
            } else {
                (center_y.checked_sub(y).unwrap()) as f32
            };

            let distance = f32::sqrt(x_dist.powi(2) + y_dist.powi(2));
            let i = (x * 4) + (y * PITCH);

            let buffer = 1_f32;
            let stroke = radius - distance;
            if distance < radius - buffer {
                pixels_as_u8[i] = 0;
                pixels_as_u8[1 + i] = 255;
                pixels_as_u8[2 + i] = 0;
                pixels_as_u8[3 + i] = 0;
            } else if distance <= radius + buffer {
                pixels_as_u8[i] = 0;
                pixels_as_u8[1 + i] = linear2srgb(stroke);
                pixels_as_u8[2 + i] = 0;
                pixels_as_u8[3 + i] = 0;
            } else {
                // explicitly set any other pixels to black
                pixels_as_u8[i] = 0;
                pixels_as_u8[1 + i] = 0;
                pixels_as_u8[2 + i] = 0;
                pixels_as_u8[3 + i] = 0;
            }
        }
    }
}
