use std::{fs, io::Write, num::TryFromIntError};

struct RGB {
    r : u8,
    g : u8,
    b : u8
}

impl RGB {
    fn from_bytes(bytes : &[u8]) -> RGB {
        RGB {
            b: bytes[0],
            g: bytes[1],
            r: bytes[2]
        }
    }
}

struct Bitmap {
    width : u32,
    height : u32,
    pixels : Vec<Vec<RGB>>
}

fn read_4_bytes(bytes : &[u8]) -> u32 {
    ((bytes[0] as u32) <<  0) +
    ((bytes[1] as u32) <<  8) +
    ((bytes[2] as u32) << 16) +
    ((bytes[3] as u32) << 24)
}

fn round_to_4_bytes(number : u32) -> Result<u32, TryFromIntError> {
    let inumber: i64 = i64::from(number);
    if number % 4 == 0 { return Ok(number * 3) }
    return u32::try_from(inumber * 3 + -((inumber * 3) % 4) + 4);
}

impl Bitmap {
    fn from(file_path: &str) -> Result<Bitmap, Box<dyn std::error::Error + 'static>> {
        let bytes = fs::read(file_path)?;
        let width = read_4_bytes(&bytes[18..22]);
        let height = read_4_bytes(&bytes[22..26]);
        let bytes_per_row = round_to_4_bytes(width)?;

        let mut pixels: Vec<Vec<RGB>> = Vec::new();
        
        for y in 0..height {
            let start = Bitmap::row_start_slice(bytes_per_row, y);
            let end = Bitmap::row_end_slice(bytes_per_row, y);
            let row_bytes = &bytes[start..end];
            
            let mut row_pixels: Vec<RGB> = Vec::new();
            
            for x in 0..width * 3{
                if x % 3 == 0 {
                    let pixel_start = (x) as usize;
                    let pixel_end = (x + 3) as usize;
                    
                    let pixel_bytes = &row_bytes[pixel_start..pixel_end];
                    
                    row_pixels.push(RGB::from_bytes(pixel_bytes));
                }
            }

            pixels.push(row_pixels);
        }
        
        pixels.reverse();
        
        Ok(Bitmap {
            width,
            height,
            pixels
        })
    }
    
    fn row_start_slice(bytes_per_row : u32, row : u32) -> usize {
        (54 + row * bytes_per_row) as usize
    }

    fn row_end_slice(bytes_per_row : u32, row : u32) -> usize {
        (54 + (row + 1) * bytes_per_row) as usize
    }

    fn get_pixel_at(&self, x : u32, y : u32) -> &RGB {
        &self.pixels[y as usize][x as usize]
    }
}

struct Platform {
    x : f32,
    y : f32,
    w : f32,
    h : f32,
    r : u8,
    g : u8,
    b : u8
}

impl Platform {
    fn new(x : u32, y : u32, scale : f32, bitmap : &Bitmap, x_offset : f32, y_offset : f32) -> Platform {
        let rgb = bitmap.get_pixel_at(x, y);
        
        Platform {
            x : (x as f32) * scale + x_offset,
            y : (y as f32) * scale + y_offset,
            w : scale,
            h : scale,
            r : rgb.r,
            g : rgb.g,
            b : rgb.b
        }
    }
    
    fn to_goose(&self) -> String {
        format!(
            "X:{};Y:{};W:{};H:{};T:1;R:{};G:{};B:{};|",
            (self.x as f32) * 0.6, (self.y as f32) * 0.6, self.w, self.h, (self.r as f32) / 255.0, (self.g as f32) / 255.0, (self.b as f32) / 255.0
        )
    }
}

fn main() {
    if let Ok(bmp) = Bitmap::from("image.bmp") {
        println!("width: {}, height: {}", bmp.width, bmp.height);
        
        let mut goose_export = String::new();
        
        for y in 0..bmp.height {
            for x in 0..bmp.width {
                let platform = Platform::new(x, y, 5.0, &bmp, 200.0, 200.0);
                goose_export.push_str(&platform.to_goose());
            }
        }
        
        let export = fs::File::create("export.goose");
        if let Ok(mut file) = export {
            let _ = file.write_all(goose_export.as_bytes());
        }
    
    } else {
        panic!("what")
    }
}
