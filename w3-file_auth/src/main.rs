use sha2::{Sha256, Digest};

fn main() {
    let mut hasher = Sha256::new();
    hasher.input(b"File Auth with SHA256");

    let result = hasher.result();

    println!("Hash: {:x}", result);
}
