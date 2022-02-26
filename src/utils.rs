use std::borrow::{Borrow, BorrowMut};
use std::collections::HashSet;
use std::ops::Deref;
use regex::Regex;
use lazy_static::lazy_static;
use std::io::{self, Read};
use std::fs::File;
use std::hash::Hash;

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}



pub fn join_str(vec: Vec<&String>) -> String {
    let mut ret = String::from("");

    for i in 0..vec.len() {
        ret += " ";
        ret += vec[i];
    }

    let ret_trim = ret.trim();
    return ret_trim.to_string()
}

pub fn join_space(a: &String, b: &String) -> String {
    let ret = a.to_owned() + " " + b;

    return ret.trim().to_string()
}

pub fn return_larger_smaller<'a>(a: &'a String, b: &'a String) -> (&'a String, &'a String) {
    let len_a = a.len() as i32;
    let len_b = b.len() as i32;

    let diff = len_a - len_b;

    let larger: &String;
    let smaller: &String;

    if diff >= 0 {
        larger = a;
        smaller = b;
    } else {
        larger = b;
        smaller = a;
    }



    return (larger, smaller)


}



pub fn rem_punct_split(text_given: &str) -> HashSet<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[.,\\/#!\\?$%\^&\*;:{}=\\-_`~()]+").unwrap();
        static ref SW: String = filename_to_string("src/stopwords.txt").unwrap();
    }


    let sw_hashset: HashSet<String> = SW.
        lines()
        .into_iter()
        .map(|x| x.trim().to_string())
        .collect::<HashSet<_>>();

    let mut text: String = text_given.to_lowercase().clone();

    let mut rep: String = RE.replace_all(&text, "").to_string();

    let split = rep.split_whitespace()
                .map(|x| x.trim().to_string()).collect::<HashSet<String>>();

    let diff = split.difference(&sw_hashset).map(|x| x.to_string()).collect::<HashSet<_>>();

    return diff


}