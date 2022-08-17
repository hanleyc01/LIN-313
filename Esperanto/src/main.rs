#![allow(non_snake_case, dead_code, unused)]

mod spell;

use crate::spell::*;

use std::{
    fs::File,
    io::{Error, Read},
};

/// CSV data set, entries delimited by `:` (as `,`'s are present in data)
const DATA: &str = "csv-data.csv";
const CORPUS: &str = "esp-corpus.txt";

fn main() -> Result<(), Error> {
    // open file
    // let mut f = File::open(DATA)?;
    let s = Spell::new(CORPUS.to_string())?;

    // Planned to do some in-depth data sciences stuff, but that would take too long, and be
    // overkill given the dataset. So instead, I'm just going to print out the relevant information

    // correctly identified words
    let words1 = vec![
        "bakulo",
        "gajo",
        "hakili",
        "ĥimeriĝo",
        "hximerigxo",
        "ĵetilo",
        "jxetilo",
        "laboristo",
    ];

    let words2 = "fabelado";

    println!("prompted words");
    words1.iter().for_each(|w| {
        println!(
            "word: {}, correction: {:?}, probability: {}",
            w,
            s.check_word(w.to_string()),
            s.get_probability(w.to_string())
        );
    });

    println!("unprompted words");
    println!(
        "word: {}, correction: {:?}, probability: {}",
        words2,
        s.check_word(words2.to_string()),
        s.get_probability(words2.to_string())
    );

    // read to string buffer
    // let mut raw_data = String::new();
    // f.read_to_string(&mut raw_data);

    // push raw_data into vector
    // let mut data: Vec<&str> = Vec::new();
    // raw_data.split(':').for_each(|s| data.push(s));

    Ok(())
}
