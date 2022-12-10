use rustbitmap::BitMap;
use rustbitmap::Rgba;
use rand::Rng;

fn main() {
    
    let session_id_x = rand::thread_rng().gen_range(0..255);
    let session_id_y = rand::thread_rng().gen_range(0..255);
    let session_id_z = rand::thread_rng().gen_range(0..255);
    
    let red = random_color();
    let green = random_color();
    let blue = random_color();

    let pixels = random_pixel::<Rgba>(red, green, blue);

    let mut new_random_bitmap = create_bitmap();
    new_random_bitmap.fill_region(5, 5, green).unwrap();

    let green_test_id = format!("green_test-{}-{}-{}.bmp", session_id_x, session_id_y, session_id_z);

    new_random_bitmap.save_as(&green_test_id).unwrap();

    let mut second_random_bitmap = BitMap::create(2, 2, pixels).unwrap();
    second_random_bitmap.save_as("random_bitmap.bmp").unwrap();


}

fn create_bitmap()-> BitMap {
    let height = rand::thread_rng().gen_range(0..100);
    let width = rand::thread_rng().gen_range(0..100);
    let new_bitmap = BitMap::new(height, width);
    new_bitmap
}

fn random_color()-> Rgba {
    let red = rand::thread_rng().gen_range(0..255);
    let green = rand::thread_rng().gen_range(0..255);
    let blue = rand::thread_rng().gen_range(0..255);

    let new_random_color = Rgba::rgb(red, green, blue);
    new_random_color 
}

fn random_pixel<T>(red: Rgba, green: Rgba, blue: Rgba)-> Vec<Rgba> {
   let white = Rgba::rgb(255, 255, 255);
   let new_random_pixel = vec![red, blue, green, white];
   new_random_pixel 
}