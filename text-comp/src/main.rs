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

    let wds: Vec<&str> = txt.split_whitespace().collect();
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

fn get_avg_word_prob(wds: WdUseProb) -> f32 {
    let mut avg = 0.0;
    for (w,p) in &wds {
        avg += p;
    }
    avg / wds.len() as f32
}

fn sentences(txt: &str) -> Vec<&str> {
    txt.split('.').collect()
}

fn get_avg_l(wds: Vec<&str>) -> f32 {
    let mut sum_l = 0;
    wds.iter().for_each(|s| { sum_l += s.len(); });
    (sum_l / wds.len()) as f32
}

fn w_probs(re: &Regex, txt: String) -> HashBag<String> {
    let wds: Vec<&str> = txt.split_whitespace().collect();

    let mut bag = HashBag::new();
    for w in wds {
       let wd = w.to_lowercase();
       if re.is_match(&wd) {
           bag.insert(wd);
       }
    }
    bag
}

fn old_new(re: &Regex, old: &HashBag<String>, new: &HashBag<String>, txt: String) -> (f32,f32) {
    let wds: Vec<&str> = txt.split_whitespace().collect();
    
    let mut final_wd: Vec<String> = Vec::new();
    for w in wds {
        let wd: String = w.to_lowercase();
        if re.is_match(&wd) {
            final_wd.push(wd);
        }
    }

    println!("{:?}", final_wd);
    println!("{:?}", old);

    // old avgs
    let mut o_t: f32 = 0.;
    let o_len = old.len() as f32;
    for w in &final_wd {
        let w_avg = old.contains(&*w) as f32 / o_len;
        o_t += w_avg;
    }
    let o_a = o_t / final_wd.len() as f32;

    let mut n_t:f32 = 0.;
    let n_len = new.len() as f32;
    for w in &final_wd {
        n_t += (new.contains(&*w) as f32 / n_len);
    }
    let n_a = n_t / final_wd.len() as f32;

    (o_a, n_a)
}

fn main() -> Result<(), Error> {
    // Open the files
    let mut f1 = File::open("text1.txt")?;
    let mut f2 = File::open("text2.txt")?;
    let mut f3 = File::open("text3.txt")?;

    let mut old = File::open("darwin.txt")?;
    let mut new = File::open("modern.txt")?;

    // Allocate mutable-length string buffers
    let mut txt1 = String::new();
    let mut txt2 = String::new();
    let mut txt3 = String::new();

    let mut ot = String::new();
    let mut nt = String::new();

    // Read to string
    f1.read_to_string(&mut txt1)?;
    f2.read_to_string(&mut txt2)?;
    f3.read_to_string(&mut txt3)?;

    old.read_to_string(&mut ot)?;
    new.read_to_string(&mut nt)?;
    

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
    
    // avg probability that a word will appear in one of three texts
    let avg_wd_prob1 = get_avg_word_prob(txt1_prob);
    let avg_wd_prob2 = get_avg_word_prob(txt2_prob);
    let avg_wd_prob3 = get_avg_word_prob(txt3_prob);

    println!("{:?}", avg_wd_prob1);
    println!("{:?}", avg_wd_prob2);
    println!("{:?}", avg_wd_prob3);

    // Get avg sentence length
    let snt1 = sentences(&txt1);
    let snt2 = sentences(&txt2);
    let snt3 = sentences(&txt3);

    let avg_l1 = get_avg_l(snt1);
    let avg_l2 = get_avg_l(snt2);
    let avg_l3 = get_avg_l(snt3);

    println!("{}", avg_l1);
    println!("{}", avg_l2);
    println!("{}", avg_l3);

    // Attempt to estimate either period written
    let op = w_probs(&regex, ot);
    let np = w_probs(&regex, nt);
    
    let prob1 = old_new(&regex, &op, &np, txt1);
    let prob2 = old_new(&regex, &op, &np, txt2);
    let prob3 = old_new(&regex, &op, &np, txt3);

    println!("{:?}", prob1);
    println!("{:?}", prob2);
    println!("{:?}", prob3);

    Ok(())
}
