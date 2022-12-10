use rand::Rng;
use rustbitmap::BitMap;
use rustbitmap::Rgba;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    
    let mut _new_random_bitmap = BitMapSetup::create();

    //new_random_bitmap.fill_region(5, 5, random_rgb).unwrap();
    //new_random_bitmap.save_as(&random_id).unwrap();
    let random_rgb = random_color();
    _new_random_bitmap.random_bmp.fill_region(5,5, random_rgb);
    _new_random_bitmap.random_bmp.save_as(&_new_random_bitmap.random_file_name);
    Ok(())
}

// TO DO - place inside impl of a pub struct RandomBitMap


pub struct TwoDimensionalScalar(u32, u32);

pub struct BitMapSetup {
    pub random_file_name: String,
    pub random_height_width: TwoDimensionalScalar,
    pub random_bmp: BitMap,
}

impl BitMapSetup {
    pub fn create() -> BitMapSetup {
    let random_id_x = &rand::thread_rng().gen_range(0..255);
    let random_id_y = &rand::thread_rng().gen_range(0..255);
    let random_id_z = &rand::thread_rng().gen_range(0..255);

    let random_file_name = format!("{}-{}-{}.bmp", random_id_x, random_id_y, random_id_z);
    
    let height = rand::thread_rng().gen_range(1..240);
    let width = rand::thread_rng().gen_range(1..120);

    let random_height_width: TwoDimensionalScalar = TwoDimensionalScalar(height, width);
    
    let random_bmp = random_bitmap(&random_height_width);

    BitMapSetup { random_file_name, random_height_width, random_bmp }
    }   
}

pub fn random_bitmap(random_scalar: &TwoDimensionalScalar) -> BitMap {
    let new_bitmap = BitMap::new(random_scalar.0, random_scalar.1);
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