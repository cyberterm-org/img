use std::{env, fs::File, io::Read, path::Path};

fn main() {
    if let Some(path) = env::args().nth(1) {
        let path = Path::new(&path);
        let guess = mime_guess::from_path(path);

        let mut file = File::open(&path).unwrap();
        let mut buffer = Vec::new();

        let _out = file.read_to_end(&mut buffer);

        let _size = match _out {
            Ok(size) => size,
            Err(error) => panic!("read file error:{}", error),
        };
        let base64 = base64::encode(buffer);
        let mime_type = guess.first().unwrap();
        println!("data:image/{};base64,{}", mime_type.to_string(), base64);
    } else {
        panic!("You must provide a file path");
    }
}
