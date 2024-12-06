use std::fs;
use std::num::TryFromIntError;

pub struct RGB {
    pub r : u8,
    pub g : u8,
    pub b : u8
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

pub struct Bitmap {
    pub width : u32,
    pub height : u32,
    pixels : Vec<Vec<RGB>>
}

fn read_4_bytes(bytes : &[u8]) -> u32 {
    bytes.iter().enumerate().map(|(i, &b)| (b as u32) << (i * 8)).sum()
}

fn round_to_4_bytes(number : u32) -> Result<u32, TryFromIntError> {
    let inumber: i64 = i64::from(number);
    if number % 4 == 0 { return Ok(number * 3) }
    return u32::try_from(inumber * 3 + -((inumber * 3) % 4) + 4);
}

impl Bitmap {
    pub fn from(file_path: &str) -> Result<Bitmap, Box<dyn std::error::Error + 'static>> {
        let bytes = fs::read(file_path)?;
        let width = read_4_bytes(&bytes[18..22]);
        let height = read_4_bytes(&bytes[22..26]);
        let bytes_per_row = round_to_4_bytes(width)?;

        let mut pixels: Vec<Vec<RGB>> = Vec::new();
        
        println!("Parsing Bitmap...");

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

        println!("Parsed Bitmap");
        
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

    pub fn get_pixel_at(&self, x : u32, y : u32) -> &RGB {
        &self.pixels[y as usize][x as usize]
    }
}