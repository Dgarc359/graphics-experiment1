#[allow(dead_code, unused_variables, unused_braces)]
extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color,PixelFormatEnum};
use sdl2::event::Event;
use std::time::Duration;
use sdl2::rect::{Rect, Point};
const WIDTH: usize = 256;
const HEIGHT: usize = 240;
const PITCH: usize = std::mem::size_of::<u32>() * WIDTH;


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();


    let window = video_subsystem.window("rust-sdl2 demo", 720, 480)
        .position_centered()
        .build()
        .unwrap();


    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let texture_creator = canvas.texture_creator();

    // create textures
    let mut test = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGBA8888,
        WIDTH as u32,
        HEIGHT as u32,
    ).unwrap();


    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {

        //const arr_width: usize = usize::try_from(WIDTH).unwrap() * 4;

        let pixels_as_u8: &[u8] = &[255; 1024];
        test.update(None, pixels_as_u8, PITCH).expect("couldnt copy raw pixels");

        canvas.copy(&test, None, None).expect("couldnt copy texture to canvas");

        canvas.present();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => { }
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

}
