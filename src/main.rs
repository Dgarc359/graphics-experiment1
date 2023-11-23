extern crate sdl2;

mod assets;
mod game;
mod util;
use std::env;
use std::path::Path;
use util::linear2srgb;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use std::time::Duration;

use crate::game::ExternalMinesweeper;
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
    let mine_img_path = Path::new(pwd_str).join("src").join("mine.png");
    let has_mine_img_path = Path::new(pwd_str).join("src").join("has_mine.png");

    println!("{:?}", mine_img_path.to_str().unwrap());
    let block = texture_creator.load_texture(mine_img_path).unwrap();
    let mine = texture_creator.load_texture(has_mine_img_path).unwrap();
    let mut game = game::new_game(10, 10, 3);
    let padding = 60_i32;
    for y in 0..game.get_height() {
        for x in 0..game.get_width() {
            let rect = Rect::new(
                (x as i32 * padding) + 100,
                (y as i32 * padding) + 100,
                50,
                50,
            );
            if game.explore_tile(x, y) {
                canvas.copy(&mine, None, rect).unwrap();
            } else {
                canvas.copy(&block, None, rect).unwrap();
            }
        }
    }
    //let rect = Rect::new(10, 10, 50, 50);
    //canvas.copy(&texture, None, rect).unwrap();
    canvas.present();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown {
                    timestamp,
                    window_id,
                    which,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                } => {
                    println!("x: {} y: {}", x, y);
                }
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn make_circle(radius: f32, pixels_as_u8: &mut [u8]) {
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
