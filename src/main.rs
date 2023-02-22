use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> io::Result<()> {
    #[cfg(debug_assertions)]
    let (input, output, error) = (
        File::open("input.txt")?,
        File::create("output.txt")?,
        File::create("error.txt")?,
    );

    #[cfg(debug_assertions)]
    let (input, output, error) = (
        io::BufReader::new(input),
        io::BufWriter::new(output),
        io::BufWriter::new(error),
    );

    #[cfg(not(debug_assertions))]
    let (input, output, error) = (io::stdin(), io::stdout(), io::stderr());

    #[cfg(debug_assertions)]
    let start = SystemTime::now();

    let mut input = input.lock();
    let mut output = output.lock();
    let mut error = error.lock();

    let mut t = String::new();
    input.read_line(&mut t)?;
    let t: usize = t.trim().parse().unwrap();

    for _ in 0..t {
        writeln!(error, "-----------Compile Time----------")?;
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        writeln!(error, "{}", current_time)?;
        writeln!(error, "-----------------start-----------")?;
        solve(&mut input, &mut output)?;
        writeln!(error, "-----------------end-------------")?;
    }

    let duration = match start.elapsed() {
        Ok(elapsed) => elapsed.as_secs_f32(),
        Err(e) => panic!("Error: {:?}", e),
    };

    writeln!(error, "time taken : {} secs", duration)?;
    Ok(())
}

fn solve(input: &mut dyn BufRead, output: &mut dyn Write) -> io::Result<()> {
    // Implement the solution logic here
    Ok(())
}
