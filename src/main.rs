use std::{fs, num::TryFromIntError};

struct Bitmap {
    width : u32,
    bytes_per_row : u32,
    height : u32,
    bytes : Vec<u8>
}

struct RGB {
    r : u8,
    g : u8,
    b : u8
}

impl RGB {
    fn from_bytes(bytes : &[u8]) -> RGB {
        RGB {
            r: bytes[0],
            g: bytes[1],
            b: bytes[2]
        }
    }
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

        Ok(Bitmap {
            width,
            height,
            bytes_per_row,
            bytes
        })
    }

    fn row_start_slice(&self, row : u32) -> usize {
        (54 + row * self.bytes_per_row) as usize
    }

    fn row_end_slice(&self, row : u32) -> usize {
        (54 + (row + 1) * self.bytes_per_row) as usize
    }

    fn get_pixels(&self) -> Vec<Vec<RGB>> {
        let mut pixels: Vec<Vec<RGB>> = Vec::new();

        for y in 1..self.height {
            let start = self.row_start_slice(y);
            let end = self.row_end_slice(y);
            let row_bytes = &self.bytes[start..end];
            
            let mut row_pixels: Vec<RGB> = Vec::new();

            for x in 0..self.width {
                if x % 3 == 0 {
                    let pixel_start = (x * 3) as usize;
                    let pixel_end = ((x + 1) * 3) as usize;
                    let pixel_bytes = &row_bytes[pixel_start..pixel_end];
    
                    row_pixels.push(RGB::from_bytes(pixel_bytes));
                }
            }

            pixels.push(row_pixels);
        }

        return pixels;
    }
}

fn main() {
    if let Ok(bmp) = Bitmap::from("image.bmp") {
        println!("width: {}, height: {}", bmp.width, bmp.height);

        let pixels = bmp.get_pixels();
        for y in pixels.iter() {
            for x in y.iter() {
                println!("R : {}, B : {}, G : {}, X : {}, Y : {}", x.r, x.g, x.b, 0, 0)
            }
        }

    } else {
        panic!("what")
    }
}
