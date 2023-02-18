use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::num::NonZeroUsize;

fn main() -> io::Result<()> {
    let args = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .arg(
            clap::Arg::with_name("size-hint")
                .long("size-hint")
                .short("s")
                .takes_value(true)
                .validator(|v| {
                    v.parse::<usize>()
                        .map(|_| ())
                        .map_err(|e| format!("invalid number: {}", e))
                })
                .help("how much space to pre-allocate"),
        )
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

    let size_hint = args
        .value_of("size-hint")
        .and_then(|v| v.parse().ok())
        .and_then(NonZeroUsize::new);

    let stdin = io::stdin();
    let stdin = stdin.lock();

    let stdout = io::stdout();
    let stdout = stdout.lock();

    if args.is_present("local") {
        let default = NonZeroUsize::new(32).expect("static constant");
        local_uniq(stdin, stdout, size_hint.unwrap_or(default))
    } else if args.is_present("count") {
        flat_count(stdin, stdout, size_hint.map(|v| v.get()).unwrap_or(10_000))
    } else {
        stable_uniq(stdin, stdout, size_hint.map(|v| v.get()).unwrap_or(10_000))
    }
}

fn local_uniq<R: BufRead, W: Write>(
    from: R,
    mut to: W,
    view_distance: NonZeroUsize,
) -> io::Result<()> {
    let mut seen = lru::LruCache::new(view_distance);

    for line in from.lines() {
        let line = line?;
        if seen.contains(&line) {
            continue;
        }
        writeln!(to, "{}", line)?;
        seen.put(line, ());
    }

    Ok(())
}

fn flat_count<R: BufRead, W: Write>(from: R, mut to: W, size_hint: usize) -> io::Result<()> {
    let mut count: HashMap<String, u64> = HashMap::with_capacity(size_hint);

    for line in from.lines() {
        let line = line?;
        *count.entry(line).or_insert(0) += 1;
    }

    let mut vec: Vec<(String, u64)> = count.into_iter().collect();
    vec.sort_by_key(|&(_, count)| count);
    for (line, count) in vec {
        writeln!(to, "{:10} {}", count, line)?;
    }

    Ok(())
}

fn stable_uniq<R: BufRead, W: Write>(from: R, mut to: W, size_hint: usize) -> io::Result<()> {
    let mut seen = HashSet::with_capacity(size_hint);

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
    use std::num::NonZeroUsize;

    fn run_local(input: &[u8], view_distance: usize) -> String {
        let mut out = Vec::with_capacity(input.len() / 8);
        let input = io::Cursor::new(input);
        super::local_uniq(
            input.clone(),
            &mut out,
            NonZeroUsize::new(view_distance).expect("test input"),
        )
        .unwrap();
        String::from_utf8(out).unwrap()
    }

    #[test]
    fn local() {
        let one = "a\nb\nc\nd\n";
        let two = format!("{0}{0}", one);
        for view_distance in 1..=3 {
            assert_eq!(two, run_local(two.as_bytes(), view_distance));
        }
        for view_distance in 4..=9 {
            assert_eq!(one, run_local(two.as_bytes(), view_distance));
        }
    }
}
