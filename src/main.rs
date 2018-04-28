extern crate clap;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn main() {
    let args = clap::App::new("sortuniq")
        .arg(clap::Arg::with_name("count").short("c"))
        .arg(clap::Arg::with_name("local"))
        .get_matches();
    if args.is_present("count") {
        flat_count()
    } else {
        stable_uniq()
    }
}

fn flat_count() -> () {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut count: HashMap<String, u64> = HashMap::with_capacity(10_000);

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

fn stable_uniq() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut seen = HashSet::with_capacity(10_000);

    for line in stdin.lines() {
        let line = line.expect("read error in a line");
        // TODO: This is probably inefficient; probably moves the new value into the map,
        // TODO: and frees the old one. Maybe swap for `contains()`, or manual map.
        if seen.insert(line.clone()) {
            println!("{}", line)
        }
    }

}