use std::env;
use std::fs::File;
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
        println!("Error: File path argument is missing");
        process::exit(1);
    }
    let filename = &args[0];

    let path = Path::new(filename);
    let file_iter = FileRevIter::new(&path)?;

    println!("File size: {}", file_iter.filesize);

    let mut hash = None;

    // Iterates file in from last block to first
    for (mut len, mut buf) in file_iter {
        if let Some(val) = hash {
            buf.extend(&val);
            len = buf.len();
        }

        hash = Some(Sha256::digest(&buf[0..len]));
    }

    if let Some(val) = hash {
        println!("Hash: {:x}", val);
    }

    Ok(())
}
