use std::env;
use std::fs;
use std::path::Path;
use std::process;

use sha2::{Sha256, Digest};

fn main() -> std::io::Result<()> {
    let args: Vec<_> = env::args_os().skip(1).collect();
    if args.len() < 1 {
        println!("Error: File path argument is missing");
        process::exit(1);
    }
    let filename = &args[0];

    let path = Path::new(filename);
    let size = fs::metadata(path)?.len();

    println!("File size: {}", size);

    let mut hasher = Sha256::new();
    hasher.input(b"File Auth with SHA256");

    let result = hasher.result();

    println!("Hash: {:x}", result);

    Ok(())
}
