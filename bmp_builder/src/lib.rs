use rand::Rng;
use rustbitmap::BitMap;
use rustbitmap::Rgba;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    // unique identifier for .bmp file
    let session_id_x = &rand::thread_rng().gen_range(0..255);
    let session_id_y = &rand::thread_rng().gen_range(0..255);
    let session_id_z = &rand::thread_rng().gen_range(0..255);

    let random_rgb = random_color();

    let mut new_random_bitmap = random_bitmap();
    new_random_bitmap.fill_region(5, 5, random_rgb).unwrap();

    let random_id = format!("{}-{}-{}.bmp", session_id_x, session_id_y, session_id_z);
    new_random_bitmap.save_as(&random_id).unwrap();

    Ok(())
}

// TO DO - place inside impl of a pub struct RandomBitMap
pub fn random_bitmap() -> BitMap {
    let height = rand::thread_rng().gen_range(1..240);
    let width = rand::thread_rng().gen_range(1..120);
    let new_bitmap = BitMap::new(height, width);
    new_bitmap
}

pub fn random_color() -> Rgba {
    let red = rand::thread_rng().gen_range(0..255);
    let green = rand::thread_rng().gen_range(0..255);
    let blue = rand::thread_rng().gen_range(0..255);

    let new_random_color = Rgba::rgb(red, green, blue);
    new_random_color
}

pub fn random_pixel<T>(red: Rgba, green: Rgba, blue: Rgba) -> Vec<Rgba> {
    let new_random_color = random_color();
    let new_random_pixel = vec![red, blue, green, new_random_color];
    new_random_pixel
}
