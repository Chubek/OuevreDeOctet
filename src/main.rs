mod bigram;
mod encoder;
mod levenshtein;
mod utils;
use std::default;

#[test]
fn test_bigram() {
    let bigram = bigram::new_bigram("hello2", "hlllo1", 1u32, 3u32, 4usize);
    println!("{:#?}", bigram)
}

fn main() {
    println!("Hello, world!");
}
