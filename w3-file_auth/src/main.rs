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

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args_os().skip(1).collect();
    if args.len() < 1 {
        println!("Error: File path argument is missing");
        process::exit(1);
    }
    let filename = &args[0];

    let path = Path::new(filename);
    let mut f = File::open(path)?;
    let metadata = f.metadata()?;
    let filesize = metadata.len();

    println!("File size: {}", filesize);

    let mut offset = (filesize % KB) as i64;
    let mut hash = None;

    while offset <= filesize as i64 {
        f.seek(SeekFrom::End(-offset))?;

        let mut buf = vec![0; DEFAULT_BUF_SIZE];
        let mut len = match f.read(&mut buf) {
            Ok(len) => len,
            Err(e) => return Err(e),
        };

        if let Some(val) = hash {
            buf.extend(&val);
            len = buf.len();
        }

        hash = Some(Sha256::digest(&buf[0..len]));

        offset += 1024;
    }

    if let Some(val) = hash {
        println!("Hash: {:x}", val);
    }

    Ok(())
}
