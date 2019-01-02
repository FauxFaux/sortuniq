use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::io::Write;

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

    let stdin = io::stdin();
    let stdin = stdin.lock();

    let stdout = io::stdout();
    let stdout = stdout.lock();

    if args.is_present("local") {
        local_uniq(stdin, stdout)
    } else if args.is_present("count") {
        flat_count(stdin, stdout)
    } else {
        stable_uniq(stdin, stdout)
    }
}

fn local_uniq<R: BufRead, W: Write>(from: R, mut to: W) -> io::Result<()> {
    let mut seen = lru::LruCache::new(8);

    for line in from.lines() {
        let line = line?;
        if let Some(()) = seen.get(&line) {
            continue;
        }
        writeln!(to, "{}", line)?;
        seen.put(line, ());
    }

    Ok(())
}

fn flat_count<R: BufRead, W: Write>(from: R, mut to: W) -> io::Result<()> {
    let mut count: HashMap<String, u64> = HashMap::with_capacity(10_000);

    for line in from.lines() {
        let line = line?;
        *count.entry(line).or_insert(0) += 1;
    }

    let mut vec: Vec<(String, u64)> = count.into_iter().collect();
    vec.sort_by_key(|&(_, v)| v);
    for (k, v) in vec {
        writeln!(to, "{:10} {}", v, k)?;
    }

    Ok(())
}

fn stable_uniq<R: BufRead, W: Write>(from: R, mut to: W) -> io::Result<()> {
    let mut seen = HashSet::with_capacity(10_000);

    for line in from.lines() {
        let line = line?;
        if seen.contains(&line) {
            continue;
        }
        writeln!(to, "{}", line)?;
        seen.insert(line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io;

    #[test]
    fn local() {
        let mut out = Vec::new();
        super::local_uniq(io::Cursor::new("foo\nbar\n"), &mut out).unwrap();
        assert_eq!("foo\nbar\n", String::from_utf8(out).unwrap());
    }
}
