use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::io::Write;

use measure_time::error_time;
use measure_time::log_enabled;
use measure_time::log_time;

fn main() -> io::Result<()> {
    env_logger::init();
    error_time!("main");

    let stdin = io::stdin();
    let stdin = stdin.lock();

    let stdout = io::stdout();
    let stdout = stdout.lock();

    flat_count(stdin, stdout, 10_000)
}

fn flat_count<R: BufRead, W: Write>(from: R, mut to: W, size_hint: usize) -> io::Result<()> {
    let mut count: HashMap<String, u64> = HashMap::with_capacity(size_hint);

    {
        error_time!("lines");
        for line in from.lines() {
            let line = line?;
            *count.entry(line).or_insert(0) += 1;
        }
    }

    let mut vec: Vec<(String, u64)>;
    {
        error_time!("vec");
        vec = count.into_iter().collect();
        vec.sort_by_key(|&(_, count)| count);
    }
    {
        error_time!("print");
        let mut sum = 0;
        for (line, count) in vec {
            sum += count + line.len() as u64;
        }
        writeln!(to, "{}", sum)?;
    }

    Ok(())
}
