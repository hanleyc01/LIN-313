#![allow(dead_code, unused)]

use std::{fs::File, io::{self, Read}, collections::HashMap};

// ultimately, the goal is to produce some sort of Markov chain given textual data
// process:
// 1. collect data into sentences
// 2. convert sentences into words
// 3. calculate n-gram probabilities for each word,
//    and store it in some data structure
// 4. generate possible sentences

type Unigrams = HashMap<String,Vec<(String, f32)>>;

fn get_ngram_data(uni: &mut Unigrams, snts: &Vec<String>) {
    snts.iter().for_each(|snt| {
        
        let wds: Vec<&str> = snt.split_whitespace().collect();
        for w in wds {
            
        }
    });
}

fn main() -> Result<(), io::Error> {
    // corpus of english sentences
    let fname = "example.txt";
    let mut f = File::open(fname)?;
    
    // read sentences into a list of strings
    let mut wds = String::new();
    f.read_to_string(&mut wds)?;

    // new sentence vector which contains every sentence
    // in the corpus
    let mut sentences: Vec<String> = Vec::new();
    wds.lines().for_each(|w| sentences.push(w.to_string()));
    
    println!("{:?}", sentences);

    // Unigram hashmap matches key of string to (String, f32) pair
    let mut unigrams: Unigrams = HashMap::new();
    get_ngram_data(&mut unigrams, &sentences);

    Ok(())
}
