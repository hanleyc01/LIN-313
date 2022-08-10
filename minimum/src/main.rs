#![allow(dead_code, unused)]

/// Cost of inserting a character
const INS_COST: usize = 1;
/// Cost of deleting a character
const DEL_COST: usize = 1;
/// Cost of substituting a character
const SUB_COST: usize = 2;

fn main() {
    let seq1 = "numpy";
    let seq2 = "numexpr";
    
    let mut cs1 = seq1.chars();
    let mut cs2 = seq2.chars();

    let mut min_distance = 0;
    let mut operations: Vec<String> = Vec::new();

    'pseudo_recurse: loop {
        match cs1.next() {
            Some(c1) => {
            },
            None => {
                match cs2.next() {
                    None => {
                        
                    },
                    Some(c2) => {
                    },
                }
            },
        }
    }
}
