use hex;
use sha2::{Digest, Sha256};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

fn main() {
    let g = String::from("test");
    println!("Hash {}: {}", g, g.default_hash());
    println!("SHA256 {}: {}", g, g.sha256());
}

trait Hashable {
    fn default_hash(&self) -> u64;
    fn sha256(&self) -> String;
}
impl Hashable for String {
    fn default_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
    fn sha256(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self);
        hex::encode(hasher.finalize())
    }
}
