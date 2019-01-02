use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn main() {
    let args = clap::App::new("sortuniq")
        .arg(clap::Arg::with_name("count").long("count").short("c"))
        .arg(
            clap::Arg::with_name("local")
                .long("local")
                .conflicts_with("count"),
        )
        .get_matches();
    if args.is_present("local") {
        local_uniq()
    } else if args.is_present("count") {
        flat_count()
    } else {
        stable_uniq()
    }
}

fn local_uniq() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut seen: [String; 8] = Default::default();
    let mut lines = stdin.lines();

    match lines.next() {
        Some(Ok(line)) => {
            println!("{}", line);
            seen[0] = line;
        }
        Some(Err(e)) => panic!(e),
        None => return,
    }

    let mut cursor = 1;

    for line in lines {
        let line = line.expect("read error in a line");
        if seen.contains(&line) {
            continue;
        }
        println!("{}", line);
        seen[cursor] = line;
        cursor += 1;
        if seen.len() == cursor {
            cursor = 0;
        }
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
        if seen.contains(&line) {
            continue;
        }
        println!("{}", line);
        seen.insert(line);
    }
}
