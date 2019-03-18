use std::env;
use std::fs::{OpenOptions, File};
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;
use std::process;

use sha2::{Sha256, Digest};

const KB: u64 = 1024;
const DEFAULT_BUF_SIZE: usize = 1024;

#[derive(Debug)]
struct FileRevIter {
    file: File,
    filesize: u64,
    offset: i64,
}

impl FileRevIter {
    fn new(path: &Path) -> io::Result<Self> {
        let file = File::open(path)?;
        let metadata = file.metadata()?;
        let filesize = metadata.len();
        let offset = (filesize % KB) as i64;

        Ok(FileRevIter { file, filesize, offset })
    }
}

impl Iterator for FileRevIter {
    type Item = (usize, Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset <= self.filesize as i64 {
            self.file.seek(SeekFrom::End(-self.offset)).unwrap();

            let mut buf = vec![0; DEFAULT_BUF_SIZE];
            let len = self.file.read(&mut buf).unwrap();

            self.offset += 1024;

            return Some((len, buf));
        }
        None
    }
}

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args_os().skip(1).collect();
    if args.len() < 1 {
        println!("Error: Input and output file path arguments are missing");
        process::exit(1);
    } else if args.len() < 2 {
        println!("Error: Output file path argument is missing");
        process::exit(1);
    }
    let filename = &args[0];
    let output_filename = &args[1];

    let path = Path::new(filename);
    let file_iter = FileRevIter::new(&path)?;

    println!("File size: {}", file_iter.filesize);

    let mut hash_data = Vec::new();

    // Iterates file in from last block to first
    for (mut len, mut buf) in file_iter {
        if let Some(val) = hash_data.last() {
            buf.extend(val);
            len = buf.len();
        }

        let hash = Sha256::digest(&buf[0..len]);
        hash_data.push(hash);
    }

    if let Some(val) = hash_data.last() {
        println!("Hash: {:x}", val);
    }

    let mut output_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output_filename)?;

    let mut input_file = File::open(path)?;
    let mut buf = vec![0; DEFAULT_BUF_SIZE];

    // We skip 1 because h0 is not included
    for h in hash_data.iter().rev().skip(1) {
        // Write each block appended with the hash of the next block
        let len = input_file.read(&mut buf).unwrap();
        output_file.write(&buf[0..len]).unwrap();
        output_file.write(h).unwrap();
    }
    // Write last block (no appended hash)
    let len = input_file.read(&mut buf).unwrap();
    output_file.write(&buf[0..len]).unwrap();

    Ok(())
}
