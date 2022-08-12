#![allow(dead_code, unused)]

use rand::Rng;

use std::{
    fs::File,
    io::{Error, Read},
};

const EX_FILE: &str = "example.txt";
const SEN_FILE: &str = "sentences.txt";
const BIG_FILE: &str = "big_text.txt";

/// Distinguish between a simple `Vec<String>` in that this contains `Strings` from a corpus which
/// are separated by `\n`.
type Sentences = Vec<String>;

/// Another useful type for making clear the distinction between a simple `Vec<(String, String)>`
/// and a proper `Bigram`.
type Pair = Vec<(String, String)>;

/// Takes `Sentences` and constructs a `HashBag` multiset from each two word pair; thus creating a
/// `bigram`. Return type pair is useful for constructing the probability of some Strings `s1` and
/// `s2`, `P(s1 | s2)`; or, in other words, the probability that s1 will be preceded by s2.
fn pairs(snts: &Sentences) -> Pair {
    let mut bi = Vec::new();
    snts.iter().for_each(|mut s| {
        let s_m = s.split_whitespace().collect::<Vec<&str>>();
        let s_m_len = s_m.len();
        for i in 0..s_m_len - 1 {
            bi.push((s_m[i].to_lowercase(), s_m[i + 1].to_lowercase()));
        }
    });
    bi
}

/// Given our collected bigram probabilities, or, in other words, `P(str1 | str2)`, we can look
/// into generating a string. Importantly, this will be an implementation of a [Markov
/// Chain](https://en.wikipedia.org/wiki/Markov_chain).
///
/// To put it simply, our initial and final state are pre-defined, `'^'` for the entrypoint, and
/// `'$'` for the ending character. Using these two, we can generate a state machine based off of
/// the probability of some word following another.
///
/// Thus, for example, given the string `"The dog chased the cats."`, we can generate the
/// `Bigrams`, so `[(("^", "the"), 1.0), (("the", "dog"), 0.5), (("dog", "chased"), 1.0),
/// (("chased", "the"), 1.0), (("the", "cats"), 0.5), (("cats", "$"), 1.0)]`.
///
/// This `Bigram` can then be constructed as a Markov Chain:
/// ```text
/// // Where each state is some string, and -(f32)-> is the *probability* of that state transition
/// "^" -(1.0)-> "the" -(0.5)-> "dog" -(1.0)-> "chased"
///                | ^____________(1.0)____________⌋  
///                |                    
///                |
///                ⌊_(0.5)-> "cats" -(1.0)-> "$"
/// ```
fn gen_word(bigram_probability: &Pair) -> String {
    let mut sent: Vec<String> = Vec::new();
    let mut list_possib: Vec<((String, String))> = Vec::new();
    for (l, r) in bigram_probability {
        if l.eq(&"^".to_string()) {
            list_possib.push((l.to_string(), r.to_string()));
        }
    }

    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..list_possib.len());

    let (h, t) = &list_possib[i];
    sent.push(h.to_string());

    let mut curr = t.to_string();

    while curr != "$" {
        let mut rng = rand::thread_rng();
        let mut l_p2: Vec<(String, String)> = Vec::new();
        for (l, r) in bigram_probability {
            if *l == curr {
                l_p2.push((l.to_string(), r.to_string()));
            }
        }
        let i = rng.gen_range(0..l_p2.len());

        let (h, t) = &l_p2[i];
        sent.push(h.to_string());

        curr = t.to_string();
    }

    let mut fin = String::new();
    for w in sent {
        w.chars().for_each(|c| fin.push(c));
        fin.push(' ');
    }
    fin
}

fn main() -> Result<(), Error> {
    // Open file
    let mut f = File::open(BIG_FILE)?;

    // Read file to `raw`
    let mut raw = String::new();
    f.read_to_string(&mut raw);

    raw = raw
        .replace('^', " ^ ")
        .replace('$', " $ ")
        .replace('.', "")
        .replace('[', "")
        .replace(']', "");

    // Collect "sentences", or, rather, our corpus separated by `\n` characters
    let mut sentences = Sentences::new();
    raw.lines().for_each(|s| {
        sentences.push(s.to_string());
    });

    let pairs = pairs(&sentences);

    for _ in 0..10 {
        println!("{:?}", gen_word(&pairs));
    }
    Ok(())
}
