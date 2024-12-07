use std::{fs, io::{self, Write}, path::PathBuf, time::Instant};
use rfd::FileDialog;
use rayon::prelude::*;

use bmp2goose::bitmap_parser::Bitmap;
use bmp2goose::goose_parser::Platform;

fn get_input(message : &str) -> Result<String, io::Error> {
    println!("{}", message);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn get_float_input(message : &str) -> f32 {
    loop {
        match get_input(message) {
            Ok(input) => match input.parse::<f32>() {
                Ok(value) => break value,
                Err(_) => println!("Invalid input. Please enter a valid number."),
            },
            Err(_) => println!("Failed to read input. Please try again."),
        }
    }
}

fn get_boolean_input(message : &str) -> bool {
    loop {
        match get_input(message) {
            Ok(input) => match input.as_str() {
                "y" => break true,
                "n" => break false,
                _ => println!("Please enter y or n.")
            },
            Err(_) => println!("Failed to read input. Please try again."),
        }
    }
}

fn main() {
    println!("Select a bitmap file...");
    let file_path: PathBuf = loop {
        let file = FileDialog::new()
        .add_filter("bitmap", &["bmp"])
        .set_directory("/")
        .set_title("Select a bitmap file")
        .pick_file();
        
        if let Some(path) = file {
            break path;
        }
    };
    println!("Select an export location...");
    let export_path: PathBuf = loop {
        let file = FileDialog::new()
        .add_filter("Goose Platformer", &["goose"])
        .set_file_name("export.goose")
        .set_title("Select an export location")
        .save_file();
        
        if let Some(path) = file {
            break path;
        }
    };
    
    let scale : f32 = get_float_input("Enter what size you want each pixel to be (1.0 = 1 pixel): ");
    let x_offset : f32 = get_float_input("Enter the starting X position of the image");
    let y_offset : f32 = get_float_input("Enter the starting Y position of the image");
    let remove_white_pixels : bool = get_boolean_input("Do you want to exclude white pixels? (The background in Goose Platformer is white, making these pixels practically invisible) (y/n): ");
    
    if let Ok(bmp) = Bitmap::from(file_path.to_str().unwrap()) {
        let start_time = Instant::now();

        let goose_export: String = (0..bmp.height)
        .into_par_iter()
        .map(|y| {
            let mut row_export = String::new();
            for x in 0..bmp.width {
                let pixel = bmp.get_pixel_at(x, y);
                
                if remove_white_pixels && pixel.r == 255 && pixel.g == 255 && pixel.b == 255 {
                    continue
                }
                
                let platform = Platform::new(x, y, scale, &bmp, x_offset, y_offset);
                row_export.push_str(&platform.to_goose());
            }
            row_export
        })
        .collect();
        
        let export = fs::File::create(export_path.to_str().unwrap());
        if let Ok(mut file) = export {
            let _ = file.write_all(goose_export.as_bytes());
        }

        println!("Finished conversion in {:?}", start_time.elapsed());
    
    } else {
        panic!("The file was somehow now found... Something has gone wrong!")
    }
    
    println!("");
    
    println!("Successfully exported to {}", export_path.to_str().unwrap());
    let _ = get_input("Press enter to exit...");
}
