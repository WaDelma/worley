use std::path::Path;

use image::{ImageBuffer, Rgb};

use nalgebra::Point2 as Point;
use nalgebra::Vector2 as Vector;

use rand::{rngs::SmallRng, Rng, SeedableRng};

use byteorder::{ByteOrder, LE};

fn main() {
    let mut buffer = ImageBuffer::new(1 << 8, 1 << 8);
    let scale = 0.05;
    for x in 0..buffer.width() {
        for y in 0..buffer.height() {
            let xx = x as f32 - buffer.width() as f32 / 2.;
            let yy = y as f32 - buffer.height() as f32 / 2.;
            let noise = noise(Point::new(xx * scale, yy * scale), 1000, &|v1, v2| {
                (v1 - v2).lp_norm(2)
            });
            let red = ((noise.0 * 1.75).tan() * 255.) as u8;
            let green = ((noise.0 * 6.).atan() * 255.) as u8;
            let blue = ((1. - noise.1).powf(1.2) * 255.) as u8;
            buffer[(x, y)] = Rgb([red, green, blue]);
        }
    }
    println!("Hashes: {}", unsafe { HASHES });
    buffer.save(Path::new("lol.png")).unwrap();
}

fn noise<F>(coord: Point<f32>, seed: i32, d: &F) -> (f32, f32)
where
    F: Fn(Vector<f32>, Vector<f32>) -> f32,
{
    let grid_coord = coord.coords.map(|f| f.floor() as i32);

    let mut closest = 2.;
    let mut snd_closest = 2.;
    // Hashes in naive: 589824
    for x in -1..=1 {
        for y in -1..=1 {
            let other_coord = grid_coord + Vector::new(x, y);

            let dist = closest_sample(coord, other_coord, seed, d);
            if dist < closest {
                snd_closest = closest;
                closest = dist;
            }
        }
    }
    (
        if closest > 1. { 1. } else { closest },
        if snd_closest > 1. { 1. } else { snd_closest },
    )
}

static mut HASHES: usize = 0;

fn closest_sample<F>(coord: Point<f32>, other_coord: Vector<i32>, seed: i32, d: &F) -> f32
where
    F: Fn(Vector<f32>, Vector<f32>) -> f32,
{
    unsafe {
        HASHES += 1;
    }
    let mut s = [0u8; 16];
    LE::write_i32(&mut s[0..=3], other_coord.x);
    LE::write_i32(&mut s[4..=7], other_coord.y);
    LE::write_i32(&mut s[8..=11], seed);
    let mut rng = SmallRng::from_seed(s);
    let point = rng.gen::<Vector<f32>>();

    let feature = other_coord.map(|f| f as f32) + point;
    d(coord.coords, feature)
}
