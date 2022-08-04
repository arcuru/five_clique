use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Word {
    word: String,
    bits: u32,
}

impl Word {
    fn new(word: String, bits: u32) -> Word {
        Word { word, bits }
    }
}

fn wordbits(word: &str) -> Option<u32> {
    // Construct the bitfield
    let mut bits = 0;
    for c in word.chars() {
        let newbits = bits | 1 << (c.to_ascii_lowercase() as u32 - 'a' as u32);
        if newbits == bits {
            return None;
        }
        bits = newbits;
    }
    Some(bits)
}

fn main() {
    let file = File::open("words_alpha.txt").unwrap();
    let reader = BufReader::new(file);
    let mut wordlist = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 5 {
            if let Some(bits) = wordbits(&line) {
                wordlist.push(Word::new(line, bits));
            }
        }
    }
    build_clique(0, 0, String::new(), 0, &wordlist);
}

fn build_clique(
    depth: i32,
    accumulator: u32,
    clique: String,
    nextindex: usize,
    wordlist: &Vec<Word>,
) {
    if depth == 1 && nextindex % 100 == 0 {
        // Print progress to stderr so I know it's working
        eprintln!("{} / {}", nextindex, wordlist.len());
    }
    if depth == 5 {
        println!("{}", clique);
        return;
    }
    for i in nextindex..wordlist.len() {
        if accumulator & wordlist[i].bits == 0 {
            build_clique(
                depth + 1,
                accumulator | wordlist[i].bits,
                clique.clone() + " " + &wordlist[i].word,
                i + 1,
                wordlist,
            );
        }
    }
}
