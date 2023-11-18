#[allow(dead_code, unused_variables, unused_braces)]
extern crate sdl2;

mod local_util;
use local_util::linear2srgb;


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
    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGBA8888,
        WIDTH as u32,
        HEIGHT as u32,
    ).unwrap();


    let center_x = WIDTH / 2;
    let center_y = HEIGHT / 2;
    let radius = 100f32;


    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

        let pixels_as_u8: &mut [u8] = &mut [0; RESOLUTION];

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
                if distance <= radius {

                    println!("distance {:?} radius {:?}", distance, radius);
                    let distance_percentage = 1_f32 - (distance / radius);

                    let i = (x * 4) + (y * PITCH);
                    /*
                    pixels_as_u8[i] = x as u8; // A
                    pixels_as_u8[1 + i] = (x + y) as u8; // B
                    pixels_as_u8[2 + i] = y as u8; // G
                    pixels_as_u8[3 + i] = 255; // R
                    */
                    println!("distance percent {:?}", distance_percentage);
                    pixels_as_u8[i] = 0;
                    pixels_as_u8[1 + i] = linear2srgb(distance_percentage);
                    pixels_as_u8[2 + i] = 0;
                    pixels_as_u8[3 + i] = 0;
                }
            }
        }


        let slice = &pixels_as_u8[0..100];
        println!("done with loop");
        //println!("slice: {:#?}", slice);
    'running: loop {


        texture.update(None, pixels_as_u8, PITCH).expect("couldnt copy raw pixels");

        canvas.copy(&texture, None, None).expect("couldnt copy texture to canvas");

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
