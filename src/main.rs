use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut sum = 0;

    for line in stdin.lines() {
        let line = line?;
        sum += line.len() as u64;
    }

    println!("bytes: {}", sum);

    Ok(())
}
