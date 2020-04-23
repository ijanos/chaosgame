use image;
use rand::seq::IteratorRandom;

// modify these
const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;
const SIDES: usize = 5;
const ALWAYS_PICK_NEW: bool = true;

fn main() {
    const PI: f64 = std::f64::consts::PI;

    let centerx = WIDTH / 2;
    let centery = HEIGHT / 2;
    let radius = WIDTH / 2;

    let center_angle = 2.0 *  PI / SIDES as f64;
    let start_angle = if SIDES % 2 == 0 {
        PI / 2.0 - center_angle / 2.0
    } else {
        PI / 2.0
    };

    let mut vertices = Vec::with_capacity(SIDES);

    for i in 0..SIDES {
        let ang = start_angle + (i as f64 * center_angle);
        let vx = (centerx as f64 + radius as f64 * ang.cos()).round() as u32;
        let vy = (centery as f64 - radius as f64 * ang.sin()).round() as u32;
        vertices.push((vx, vy));
    }

    let mut cursor = (300, 300);
    let mut imgarray = vec![[0; WIDTH]; HEIGHT];
    let mut last_pick = cursor;

    for _ in 0..10_000_000 {
        let mut pick;
        loop {
            pick = vertices.iter().choose(&mut rand::thread_rng()).unwrap();
            if !ALWAYS_PICK_NEW || pick != &last_pick {
                break;
            }
        }
        last_pick = *pick;
        let mx = (pick.0 + cursor.0) / 2;
        let my = (pick.1 + cursor.1) / 2;
        cursor = (mx, my);
        imgarray[mx as usize][my as usize] += 1;
    }

    let max = imgarray.iter().max_by_key(|row| row.iter().max().unwrap()).unwrap().iter().max().unwrap();

    let mut imgbuf = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
            let t = imgarray[x][y] as f32 / *max as f32;
            let c = (t * 255.0) as u8;
            *pixel = image::Rgb([0, c, c]);
        }
    }

    imgbuf.save("fractal.png").unwrap();
}