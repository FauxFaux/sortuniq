use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let args = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .arg(
            clap::Arg::with_name("count")
                .long("count")
                .short("c")
                .help("prefix lines by the number of occurrences"),
        )
        .arg(
            clap::Arg::with_name("local")
                .long("local")
                .conflicts_with("count")
                .help("filter out nearby repetitions"),
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

fn local_uniq() -> io::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut seen = lru::LruCache::new(8);

    for line in stdin.lines() {
        let line = line?;
        if let Some(()) = seen.get(&line) {
            continue;
        }
        println!("{}", line);
        seen.put(line, ());
    }

    Ok(())
}

fn flat_count() -> io::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut count: HashMap<String, u64> = HashMap::with_capacity(10_000);

    for line in stdin.lines() {
        let line = line?;
        *count.entry(line).or_insert(0) += 1;
    }

    let mut vec: Vec<(String, u64)> = count.into_iter().collect();
    vec.sort_by_key(|&(_, v)| v);
    for (k, v) in vec {
        println!("{:10} {}", v, k);
    }

    Ok(())
}

fn stable_uniq() -> io::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut seen = HashSet::with_capacity(10_000);

    for line in stdin.lines() {
        let line = line?;
        if seen.contains(&line) {
            continue;
        }
        println!("{}", line);
        seen.insert(line);
    }

    Ok(())
}
