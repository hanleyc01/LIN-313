#![allow(dead_code)]
#![allow(unused)]

use std::cmp::max;
use std::{io::Read, fs};
use std::hash::Hash;

use hashbag::HashBag;

/// Returns the total count of all elements within the HashBag/multiset
fn count<T>(bag: &HashBag<T>) -> usize where T: Eq, T: Hash {
    if bag.is_empty() { return 0 }
    let mut count = 0;
    for e in bag {
        count += bag.contains(e);
    }
    count
}

/// Generalized function over any element within some HashBag/multiset, but we can thing of this as
/// the ratio of the count of some element over the total count of all elements.
fn probability<T>(bag: &HashBag<T>, thing: T) -> f32 where T: Eq, T: Hash {
    let count = count(bag);

    bag.contains(&thing) as f32 / count as f32
}

fn main() -> Result<(), std::io::Error> {
    
    // Environment arguments
    let args: Vec<String> = std::env::args().collect();
    
    // Check for file
    let f = match args.get(1) {
        Some(file) => {
            file.to_string()
        },
        None => panic!("No arguments provided"),
    };
    
    // Get file contents to string
    let mut contents: Vec<String> = match fs::read_to_string(f) {
        Ok(s) => s.split_whitespace().map(|x| x.to_string().to_lowercase()).collect(),
        Err(e) => return Err(e),
    };
    
    // Place words into a multiset
    let mut words: HashBag<String> = HashBag::new();
    contents.iter().for_each(|word| { words.insert(word.to_string()); });
    
    // get probabilities of each word
    let mut probs: Vec<f32> = Vec::new();
    for word in contents {
        probs.push(probability(&words, word));
    }

    println!("{:?}", &words);

    Ok(())
}
