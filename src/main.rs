use image::io::Reader as ImageReader;
use std::thread;
// create an RGB struct (8 bytes each (0-255 to represent each color in 8bit images))
#[derive(Clone)]
struct RGB {
    r: u8,
    b: u8,
    g: u8,
}

// create an Image struct (width/height can be upto 32 bits, 4 billion range (metadata), and then
// store the 2D array as 1D array because of dynamic allocation, can't give it a fixed size
// because run_time variables can't be given a fixed size, we don't know the width yet so we cant
// statically allocate it to a fixed value = [x][y])
struct Image {
    width : u32,
    height : u32,
    pixels : Vec<RGB>,
}

fn main(){
    let mut image = load_image("./input.png");
    let chunk_size = image.pixels.len() / 4;

    grayscale(&mut image, chunk_size);
    save_image(&image);
}


fn load_image(path: &str) -> Image {
    let img = ImageReader::open(path).unwrap().decode().unwrap().to_rgb8();

    let width = img.width();
    let height = img.height();
    let mut pixels: Vec<RGB> = Vec::new();

    for y in 0..height {
        for x in 0..width{
            
            let p = img.get_pixel(x,y);
            
            let rgb = RGB {r: p[0], g: p[1], b: p[2]};
            
            pixels.push(rgb);
        }
    }

    return Image {width: width, height:height, pixels : pixels};
}

fn grayscale(image: &mut Image, chunk_size: usize) {

    let chunk1 = image.pixels[0..chunk_size].to_vec();
    let chunk2 = image.pixels[chunk_size..chunk_size*2].to_vec();
    let chunk3 = image.pixels[chunk_size*2..chunk_size*3].to_vec();
    let chunk4 = image.pixels[chunk_size*3..].to_vec();
    
    let t1 = thread::spawn(move || {
        //give c the ownership of chunk1, chunk1 is deleted now
        let mut chunk = chunk1;
        process_chunk(&mut chunk);
        return chunk;
    });

    let t2 = thread::spawn(move || {
        let mut chunk = chunk2;
        process_chunk(&mut chunk);
        return chunk;
    });

    let t3 = thread::spawn(move || {
        let mut chunk = chunk3;
        process_chunk(&mut chunk);
        return chunk;
    });
    
    let t4 = thread::spawn(move || {
        let mut chunk = chunk4;
        process_chunk(&mut chunk);
        return chunk;
    });

    let result1 = t1.join().unwrap();
    let result2 = t2.join().unwrap();
    let result3 = t3.join().unwrap();
    let result4 = t4.join().unwrap();

    image.pixels = [result1, result2, result3, result4].concat();

}

fn process_chunk(chunk: &mut Vec<RGB>) {
    for pixel in chunk.iter_mut(){
        let avg = ((pixel.r as u16 + pixel.g as u16 + pixel.b as u16) / 3) as u8;
        pixel.r = avg;
        pixel.g = avg;
        pixel.b = avg;
    }
}

fn save_image(image: &Image) {
    
    let path = "./output.png";

    let mut imgbuf = image::RgbImage::new(image.width, image.height);
    for y in 0..image.height {
        for x in 0..image.width {
            let idx = (y * image.width + x) as usize;
            let rgb = &image.pixels[idx];
            imgbuf.put_pixel(x, y, image::Rgb([rgb.r, rgb.g, rgb.b]))
        }}
        
    imgbuf.save(path).unwrap();
}
