#![allow(dead_code, unused)]

use hashbag::HashBag;
use regex::Regex;

use std::{fs::File, io::{Error, Read}, collections::HashMap};

const EX_FILE: &str = "example.txt";
const SEN_FILE: &str = "sentences.txt";

/// Distinguish between a simple `Vec<String>` in that this contains `Strings` from a corpus which
/// are separated by `\n`.
type Sentences = Vec<String>;

/// Another useful type for making clear the distinction between a simple `Vec<(String, String)>`
/// and a proper `Bigram`.
type Bigrams = Vec<(String, String)>;

/// Takes `Sentences` and constructs a `HashBag` multiset from each two word pair; thus creating a
/// `bigram`. Return type pair is useful for constructing the probability of some Strings `s1` and
/// `s2`, `P(s1 | s2)`; or, in other words, the probability that s1 will be preceded by s2.
fn pairs(snts: &Sentences) -> Bigrams {
    let mut bi = Vec::new();
    snts.iter().for_each(|s| {
        let s_m = s.split_whitespace().collect::<Vec<&str>>();
        for i in 0..s_m.len()-1 {
            bi.push((s_m[i].to_lowercase(), s_m[i+1].to_lowercase()));
        }
    });
    bi
}

fn main() -> Result<(), Error> {
    
    // Open file
    let mut f = File::open(EX_FILE)?;

    // Read file to `raw`
    let mut raw = String::new();
    f.read_to_string(&mut raw);

    // Collect the sentences into words
    // This is practically just the "unigrams", albeit it does not contain the probabilities of
    // each individual unigram
    let mut words: Vec<&str> = raw.split_whitespace().collect();
    
    // store words as a multi-set, this ought to generate unigram probabilities
    let mut word_counts: HashBag<String> = HashBag::new();
    words.iter().for_each(|w| { word_counts.insert(w.to_string()); });

    // Collect "sentences", or, rather, our corpus separated by `\n` characters
    let mut sentences = Sentences::new();
    raw.lines().for_each(|s| { sentences.push(s.to_string()); });

    let pairs = pairs(&sentences);
    println!("{:?}", pairs);

    // Now, with bigrams collected into `bigrams`, we will begin storing the probability of
    // Unigrams, and Bigrams
    

    Ok(())
}
