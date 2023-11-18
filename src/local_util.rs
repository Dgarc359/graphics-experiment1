
// my local util
pub fn linear2srgb(x: f32) -> u8 {
    let x = if x <= 0.0031308 { x * 12.92 }
   else { 1.055 * x.powf(1.0/2.4) - 0.055 };
   (x * 255.0 + 0.5).floor().clamp(0.0, 255.0) as u8
}

