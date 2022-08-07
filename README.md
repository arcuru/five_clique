# Five Clique

Matt Parker recently shared a problem on Youtube [Can you find: five five-letter words with twenty-five unique letters?](https://www.youtube.com/watch?v=_-AfhLQfb6w)

In the video, he preferred to write a slow algorithm that ran for over a month. It didn't take him extra time to write, and computing time is pretty cheap, so that was good enough for him. I applaud this attitude.

He also mentions that another person, Benjamin Paassen, wrote an optimized version using graph theory that ran in 20 minutes: https://gitlab.com/bpaassen/five_clique

On Twitter today, I see that someone else spent "half-a-day" optimizing the graph theory solution to make it run in under a minute: https://github.com/phire/five_clique

## A Better Way

Matt is interested in optimizing for programmer time and effort, while the others spent time optimizing for speed.
While optimizing for speed is always fun (I see they're now down to only a handful of seconds to run their algorithm, still in Python!), Matt unfortunately never tried to see if his complicated optimizations to this problem were even worth it.

What Matt should have done is code up the simplest possible solution first, and then try to optimize it later.
In attempting to optimize prematurely, he managed to come up with a slower and more complicated algorithm, which took him several minutes of his Youtube video to describe.

**This repo took me <30 minutes to write and runs on my underpowered laptop in 10 minutes.**

It uses a naive recursive algorithm to build the solutions, with the only optimization being a pre-computed bitfield per word for comparisons.

Solutions generated are checked into the repo in the `output` file.

Here is the full algorithm, after pruning down to 5 letter words containing 5 unique characters, and precomputing the bitfields.

``` rust
struct Word {
    word: String,
    bits: u32,
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
```