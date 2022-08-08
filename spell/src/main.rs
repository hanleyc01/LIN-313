#![allow(dead_code)]
#![allow(unused)]

use hashbag::HashBag;
use regex::Regex;

use std::fs::File;
use std::hash::Hash;
use std::io::{self, Read};

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

type Splits = Vec<(String, String)>;
type Deletes = Vec<String>;
type Transposes = Vec<String>;
type Replaces = Vec<String>;
type Inserts = Vec<String>;

/// Get all possible splits of the word
fn splits(word: &String) -> Splits {
    let mut splits: Splits = Vec::new();
    for i in 0..word.len() + 1 {
        // Rust uses UTF-8 encoding, and we have already ensured that
        // the characters are ASCII (ergo, contained within a single byte)
        // by checking against `regex` below.
        let chars = word.as_bytes();
        // As said above, because Rust uses UTF-8, which is a variable length encoding, we are not
        // able to index a string directly. Thus, we must first convert to bytes, then iterate over
        // them. Note how we talk about chars *as a reference* - this is because chars' length is
        // not known at compile time.
        let head_bytes = &chars[0..i];
        let tail_bytes = &chars[i..];
        let mut head = String::new();
        let mut tail = String::new();
        // Dereferencing each byte, this again is a measure for safety, as iterators have memory
        // safety guarantees - it is impossible for us to attempt to access an element out of
        // bounds.
        head_bytes.iter().for_each(|b| head.push(*b as char));
        tail_bytes.iter().for_each(|b| tail.push(*b as char));
        splits.push((head, tail));
    }
    splits
}


/// Get all possible 1 character deletions
fn deletes(word: &String, splits: &Splits) -> Deletes {
    let mut deletes: Deletes = Vec::new();
    for (head, tail) in splits {
        // note the similar problem once again, we have to talk about the strings as being vectors
        // of bytes
        let mut car = head.as_bytes();
        let mut cdr = if tail.as_bytes().is_empty() {
            None
        } else {
            Some(tail.as_bytes())
        };
        // Some `Option` case handling for if `cdr.is_empty()` yields a true
        match cdr {
            None => {
                let mut delete = String::new();
                car.iter().for_each(|c| delete.push(*c as char));
                deletes.push(delete);
            },
            Some(bytes) => {
                let mut delete = String::new();
                let cdr2 = &bytes[1..];
                car.iter().for_each(|c| delete.push(*c as char));
                cdr2.iter().for_each(|c| delete.push(*c as char));
                deletes.push(delete);
            },
        }
    }
    deletes
}

/// All possible one-word flips
fn transposes(word: &String, splits: &Splits) -> Transposes {
    let mut transposes = Vec::new();
    for (head, tail) in splits {
        let mut car = head.as_bytes();
        let mut cdr = if tail.as_bytes().len() > 1 {
            Some(tail.as_bytes())
        } else {
            None
        };
        match cdr {
            None => {
                let mut transpose = String::new();
                car.iter().for_each(|c| transpose.push(*c as char));
                transposes.push(transpose);
            },
            Some(bytes) => {
                let mut transpose = String::new();
                let cdr0 = vec![&bytes[0]];
                let cdr1 = vec![&bytes[1]];
                let cdr2 = &bytes[2..];
                car.iter().for_each(|c| transpose.push(*c as char));
                // Double de-referencing!!
                cdr1.iter().for_each(|c| transpose.push(**c as char));
                cdr0.iter().for_each(|c| transpose.push(**c as char));
                cdr2.iter().for_each(|c| transpose.push(*c as char));
                transposes.push(transpose);
            },
        }
    }
    transposes
}

/// All possible 1 character replacements in the word
fn replaces(word: &String, splits: &Splits) -> Replaces {
    let mut replaces = Vec::new();
    for (head, tail) in splits {
        let mut car = head.as_bytes();
        let mut cdr = if tail.as_bytes().is_empty() {
            None
        } else {
            Some(tail.as_bytes())
        };
        match cdr {
            None => {
                let mut replace = String::new();
                car.iter().for_each(|c| replace.push(*c as char));
                replaces.push(replace);
            },
            Some(bytes) => {
                let cdr2 = &bytes[1..];
                for l in LETTERS.chars() {
                    let mut replace = String::new();
                    car.iter().for_each(|c| replace.push(*c as char));
                    replace.push(l);
                    cdr2.iter().for_each(|c| replace.push(*c as char));
                    replaces.push(replace);
                }
            },
        }
    }

    replaces
}

/// All possible word inserts
fn inserts(word: &String, splits: &Splits) -> Inserts {
    let mut inserts = Vec::new();
    for (head, tail) in splits {
        let car = head.as_bytes();
        let cdr = tail.as_bytes();
        for l in LETTERS.chars() {
            let mut insert = String::new();
            car.iter().for_each(|c| insert.push(*c as char));
            insert.push(l);
            cdr.iter().for_each(|c| insert.push(*c as char));
            inserts.push(insert);
        }
    }
    inserts
}

/// Get all possible 1 character edits
fn edits1(word: String) -> Vec<String> {
    let splits = splits(&word);
    let mut deletes = deletes(&word, &splits);
    let mut transposes = transposes(&word, &splits);
    let mut replaces = replaces(&word, &splits);
    let mut inserts = inserts(&word, &splits);
    
    // For inspection purposes
    println!("One deletion: {:?}", &deletes);
    // println!("One substitution: {:?}", &replaces);
    // println!("One insertion: {:?}", &inserts);

    deletes.append(&mut transposes);
    deletes.append(&mut replaces);
    deletes.append(&mut inserts);
    deletes
}

/// Return possible words
fn edits2(word: &String) -> Vec<String> {
    let mut edits = Vec::new();
    for e1 in edits1(word.to_string()) {
       for e2 in edits1(e1) {
           edits.push(e2);
       }
    }
    edits
}

/// Subset of `words` that appears in the dictionary
fn known(words: Vec<String>, bag: &HashBag<String>) -> Vec<String> {
    let mut known = Vec::new();
    for w in words {
        if bag.contains(&w) > 0 {
            known.push(w);
        }
    }
    known
}

/// Return a non-empty vector of possible words that are some edit distance away;
///
/// 1. The original word, if it is known, otherwise 
/// 2. The vector of known words one edit distance away, otherwise
/// 3. The vector of known words two edit distance away, otherwise
/// 4. The original word, even though it is not known
fn candidates(word: &String, words: &HashBag<String>) -> Vec<String> {
    
    let known_word = known(vec![word.to_string()], words);
    let known_edits1 = known(edits1(word.to_string()), words);
    let known_edits2 = known(edits2(word), words);
    let word_singleton = vec![word.to_string()];

    if !known_word.is_empty() {
        known_word
    } else if !known_edits1.is_empty() {
        known_edits1
    } else if !known_edits2.is_empty() {
        known_edits2
    } else {
        word_singleton
    }
    
}

fn correction(word: &String, words: &HashBag<String>) -> String {
    let possibilities = candidates(word, words);
    let argmax_list: Vec<f32> = possibilities.iter().map(|w| probability(words, words.len(), w)).collect();
    
    let mut max = 0.0;
    let mut max_index = 0;
    for i in 0..argmax_list.len() {
        if argmax_list[i] > max {
            max = argmax_list[i];
            max_index = i;
        }
    }

    possibilities[max_index].clone()
}

/// Returns the probability of some thing within the HashBag; i.e., the ratio of the count of some
/// object over the total count
fn probability<T: Hash + Eq>(bag: &HashBag<T>, bag_len: usize, thing: &T) -> f32 {
    bag.contains(thing) as f32 / bag_len as f32
}

fn main() -> Result<(), io::Error> {
    // Project Gutenburg text with about 1mil words
    let big_text = "./big.txt";
    let mut file = File::open(big_text)?;
    let mut big_word = String::new();
    file.read_to_string(&mut big_word);

    // Match only only words
    let regex = Regex::new(r"\w+").unwrap();

    // push to multiset
    let mut words: HashBag<String> = HashBag::new();
    big_word
        .split_whitespace()
        .filter(|word| regex.is_match(word))
        .for_each(|word| {
            words.insert(word.to_lowercase());
        });

    println!("{}",correction(&"beeds".to_string(), &words));
        
    Ok(())
}
