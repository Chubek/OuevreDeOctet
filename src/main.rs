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

#[test]
fn test_split() {
    let txt_split = utils::rem_punct_split("He? Why? Sup! Dude. Also I think this dude is... supposably astedious man! His name is JAKE THE DUDE.");

    println!("{:#?}", txt_split);

}

fn main() {
    println!("Hello, world!");
}
