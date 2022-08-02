#![allow(dead_code, unused)]

use std::fs::File;
use std::io::{Error, Read};

use hashbag::HashBag;
use regex::Regex;

type WdUseProb = Vec<(String, f32)>;

fn match_and_insert(total: &mut HashBag<String>, regex: &Regex, words: Vec<&str>) {
    for w1 in words {
        let w = w1.to_lowercase();
        if regex.is_match(&w) {
            total.insert(w);
        }
    }
}

fn get_word_totals(regex: &Regex, txt1: &str, txt2: &str, txt3: &str) -> HashBag<String> {
    let mut total = HashBag::new();

    // collect words into lists
    let wds1: Vec<&str> = txt1.split_whitespace().collect();
    let wds2: Vec<&str> = txt1.split_whitespace().collect();
    let wds3: Vec<&str> = txt1.split_whitespace().collect();

    // match words against regex and insert into hashbag
    match_and_insert(&mut total, regex, wds1);
    match_and_insert(&mut total, regex, wds2);
    match_and_insert(&mut total, regex, wds3);

    total
}

fn get_unique_prob(
    regex: &Regex,
    txt: &str,
    total_wds: &HashBag<String>,
    total_len: usize,
) -> WdUseProb {
    let mut bag: HashBag<String> = HashBag::new();

    let wds: Vec<&str> = txt.replace('-',"").collect().split_whitespace().collect();
    for wd in wds {
        let w = wd.to_lowercase();
        if regex.is_match(&w) {
            bag.insert(w);
        }
    }

    let mut useprob = Vec::new();
    for (w2, cnt) in bag {
        useprob.push((w2, cnt as f32 / total_len as f32));
    }
    useprob
}


fn main() -> Result<(), Error> {
    // Open the files
    let mut f1 = File::open("text1.txt")?;
    let mut f2 = File::open("text2.txt")?;
    let mut f3 = File::open("text3.txt")?;

    // Allocate mutable-length string buffers
    let mut txt1 = String::new();
    let mut txt2 = String::new();
    let mut txt3 = String::new();

    // Read to string
    f1.read_to_string(&mut txt1)?;
    f1.read_to_string(&mut txt2)?;
    f1.read_to_string(&mut txt3)?;

    // Get total appearances of all words
    let regex = Regex::new(r"\w+").unwrap();
    let total_words: HashBag<String> = get_word_totals(&regex, &txt1, &txt2, &txt3);
    let total_len = total_words.len();

    // Find most words most likely to use
    // get words used in text ->
    // compare probability of word being used in all three texts ->
    // find least likely words to be used
    let txt1_prob = get_unique_prob(&regex, &txt1, &total_words, total_len);
    let txt2_prob = get_unique_prob(&regex, &txt2, &total_words, total_len);
    let txt3_prob = get_unique_prob(&regex, &txt3, &total_words, total_len);

    println!("{:?}", txt1_prob);
    println!("{:?}", txt2_prob);
    println!("{:?}", txt3_prob);
    Ok(())
}
