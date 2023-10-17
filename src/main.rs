#[allow(dead_code, unused_variables, unused_braces)]
extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color,PixelFormatEnum};
use sdl2::event::Event;
use std::time::Duration;
const WIDTH: usize = 255;
const HEIGHT: usize = 255;
const PITCH: usize = 4 * WIDTH;
const RESOLUTION: usize = WIDTH * HEIGHT * 4;


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();


    let window = video_subsystem.window("rust-sdl2 demo", 1000, 1000)
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

        let pixels_as_u8: &mut [u8] = &mut [0; RESOLUTION];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                pixels_as_u8[x * y] = x as u8;
                let u8_x = x as u8;
                pixels_as_u8[x * y + 1] = u8_x.wrapping_add(y as u8);
                pixels_as_u8[x * y + 2] = y as u8;
                pixels_as_u8[x * y + 3] = 255;
            }
        }

        /*
        while i < RESOLUTION{
            pixels_as_u8[i] = 255;
            pixels_as_u8[i + 3] = 255;
            i = i + 4;
        }
        */

        let slice = &pixels_as_u8[0..100];
        println!("done with loop");
        println!("slice: {:#?}", slice);
    'running: loop {


        //let mut i = 0;
        /*
        'outer: for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if x * y + 3 >= pixels_as_u8.len()
                    || x * y + 2 >= pixels_as_u8.len()
                    || x * y + 1 >= pixels_as_u8.len() {
                    break 'outer
                }
                println!("x: {}, y: {}", x as u8, y as u8);
                let u8_x = x as u8;
                let u8_y = y as u8;
                pixels_as_u8[x * y] = u8_x;
                pixels_as_u8[x * y + 1] = u8_x.wrapping_add(u8_y);
                pixels_as_u8[x * y + 2] = u8_y;
            }
        }
        */


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
