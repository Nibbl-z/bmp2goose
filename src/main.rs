use std::fs;

struct Bitmap {
    width : u32,
    height : u32
}

fn read_4_bytes(bytes : &[u8]) -> u32 {
    ((bytes[0] as u32) <<  0) +
    ((bytes[1] as u32) <<  8) +
    ((bytes[2] as u32) << 16) +
    ((bytes[3] as u32) << 24)
}

impl Bitmap {
    fn from(file_path: &str) -> Result<Bitmap, Box<dyn std::error::Error + 'static>> {
        let bytes = fs::read(file_path)?;
        
        Ok(Bitmap {
            width : read_4_bytes(&bytes[18..22]),
            height : read_4_bytes(&bytes[22..26])
        })
    }
}

fn main() {
    if let Ok(bmp) = Bitmap::from("image.bmp") {
        println!("width: {}, height: {}", bmp.width, bmp.height);
    } else {
        panic!("what")
    }
}
