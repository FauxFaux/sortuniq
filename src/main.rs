use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::io::Write;

fn main() -> io::Result<()> {

    let stdin = io::stdin();
    let stdin = stdin.lock();

    let stdout = io::stdout();
    let stdout = stdout.lock();

    flat_count(stdin, stdout, 10_000)
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
