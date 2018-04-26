use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut count: HashMap<String, u64> = HashMap::new();

    for line in stdin.lines() {
        let line = line.expect("read error in a line");
        *count.entry(line).or_insert(0) += 1;
    }

    let mut vec: Vec<(String, u64)> = count.into_iter().collect();
    vec.sort_by_key(|&(_, v)| v);
    for (k, v) in vec {
        println!("{:10} {}", v, k);
    }
}
