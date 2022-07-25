#![allow(dead_code)]
#![allow(unused)]

use std::fs::OpenOptions;
use std::io::Write;

/// Each node in the game has a specific "rank" which is its "voting power"
#[derive(Clone)]
struct Node {
    rank: f32,
    votes: f32,
}

const BASE_RANK: f32 = 1f32 / 7f32;

impl Node {
    /// Initialize node with provided rank
    fn new(rank: f32, votes: f32) -> Self {
        Self { rank, votes }
    }

    /// Describes the division up of a node's votes per round
    fn vote(&mut self) -> f32 {
        let v = self.rank / self.votes;
        self.rank = 0.0;
        v
    }

    /// Node's update, getting 'voted'
    fn voted(&mut self, vote: f32) {
        self.rank += vote;
    }
}

fn main() -> std::io::Result<()> {
    let mut fi = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open("./results2_fixed.txt")?;

    // Initializes the nodes
    let mut a = Node::new(BASE_RANK, 1.0); // a -> b
    let mut b = Node::new(BASE_RANK, 2.0); // b -> a;
                                           // b -> d
    let mut c = Node::new(BASE_RANK, 2.0); // c -> a;
                                           // c -> b
    let mut d = Node::new(BASE_RANK, 2.0); // d -> a;
                                           // d -> c
    
    // Second question nodes
    let mut e = Node::new(BASE_RANK, 2.0); // e -> c;
                                           // e -> g
    let mut f = Node::new(BASE_RANK, 2.0); // f -> e;
                                           // f -> g
    let mut g = Node::new(BASE_RANK, 1.0); // g -> e

    let mut results: Vec<String> = Vec::new();
    for _i in 0..1000 {
        // Calculate vote per round
        let a_vote = a.vote();
        let b_vote = b.vote();
        let c_vote = c.vote();
        let d_vote = d.vote();
        let e_vote = e.vote();
        let f_vote = f.vote();
        let g_vote = g.vote();

        // b -> a; c -> a; d -> a
        a.voted(b_vote);
        a.voted(c_vote);
        a.voted(d_vote);

        // a -> b; c -> b
        b.voted(a_vote);
        b.voted(c_vote);

        // d -> c; e -> c
        c.voted(d_vote);
        c.voted(e_vote);

        // b -> d
        d.voted(b_vote);

        // f -> e; g -> e
        e.voted(f_vote);
        e.voted(g_vote);

        // e -> g; f -> g
        g.voted(e_vote);
        g.voted(f_vote);
    
        // Push to result vector
        results.push(a.rank.to_string());
        results.push(" ".to_string());
        results.push(b.rank.to_string());
        results.push(" ".to_string());
        results.push(c.rank.to_string());
        results.push(" ".to_string());
        results.push(d.rank.to_string());
        results.push(" ".to_string());
        results.push(e.rank.to_string());
        results.push(" ".to_string());
        results.push(f.rank.to_string());
        results.push(" ".to_string());
        results.push(g.rank.to_string());
        results.push("\n".to_string());
    }
    
    // Silly writing to file
    let mut results_u8: Vec<&[u8]> = Vec::new();
    results.iter().for_each(|x| results_u8.push(x.as_bytes()));
    for j in 0..results_u8.len() {
        fi.write_all(results_u8[j]);
    }
    Ok(())
}
