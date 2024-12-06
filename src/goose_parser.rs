use crate::bitmap_parser::Bitmap;

pub struct Platform {
    x : f32,
    y : f32,
    w : f32,
    h : f32,
    r : u8,
    g : u8,
    b : u8
}

impl Platform {
    pub fn new(x : u32, y : u32, scale : f32, bitmap : &Bitmap, x_offset : f32, y_offset : f32) -> Platform {
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
    
    pub fn to_goose(&self) -> String {
        format!(
            "X:{};Y:{};W:{};H:{};T:1;R:{};G:{};B:{};|",
            (self.x as f32) * 0.6, (self.y as f32) * 0.6, self.w, self.h, (self.r as f32) / 255.0, (self.g as f32) / 255.0, (self.b as f32) / 255.0
        )
    }
}