use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() {
    let file_path = "example.txt"; // Change to your file path
    load_file(file_path);
}

fn load_file(file_path: &str) {
    let path = Path::new(file_path);
    let extension = path.extension().and_then(std::ffi::OsStr::to_str).unwrap_or("");

    match extension {
        "txt" => {
            let content = load_text_file(file_path);
            println!("Text file content:\n{}", content);
        },
        "png" | "jpg" | "jpeg" => {
            let img = load_image_file(file_path);
            println!("Image file loaded with dimensions: {}x{}", img.width(), img.height());
        },
        "mp3" | "wav" => {
            let audio_file = load_audio_file(file_path);
            println!("Audio file loaded, duration: {:?}", audio_file.duration());
        },
        _ => println!("Unsupported file type"),
    }
}

fn load_text_file(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading the text file");
    contents
}

fn load_image_file(file_path: &str) -> image::DynamicImage {
    image::open(file_path).expect("Error loading the image")
}

fn load_audio_file(file_path: &str) -> audrey::read::Reader<std::io::BufReader<File>> {
    audrey::open(file_path).expect("Error loading the audio file")
}
