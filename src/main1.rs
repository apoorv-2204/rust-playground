// use std::fs::{File, OpenOptions};
// use std::io::prelude::*;
// use std::io::{self, BufRead};
// use std::time::{SystemTime, UNIX_EPOCH};

// fn main() -> io::Result<()> {
//     #[cfg(debug_assertions)]
//     let (input, output, error) = (
//         File::open("input.txt")?,
//         File::create("output.txt")?,
//         File::create("error.txt")?,
//     );

//     #[cfg(debug_assertions)]
//     let (input, output, error) = (
//         io::BufReader::new(input),
//         io::BufWriter::new(output),
//         io::BufWriter::new(error),
//     );

//     #[cfg(not(debug_assertions))]
//     let (input, output, error) = (io::stdin(), io::stdout(), io::stderr());

//     #[cfg(debug_assertions)]
//     let start = SystemTime::now();

//     let mut input = input.lock();
//     let mut output = output.lock();
//     let mut error = error.lock();

//     let mut t = String::new();
//     input.read_line(&mut t)?;
//     let t: usize = t.trim().parse().unwrap();

//     for _ in 0..t {
//         writeln!(error, "-----------Compile Time----------")?;
//         let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
//         writeln!(error, "{}", current_time)?;
//         writeln!(error, "-----------------start-----------")?;
//         solve(&mut input, &mut output)?;
//         writeln!(error, "-----------------end-------------")?;
//     }

//     let duration = match start.elapsed() {
//         Ok(elapsed) => elapsed.as_secs_f32(),
//         Err(e) => panic!("Error: {:?}", e),
//     };

//     writeln!(error, "time taken : {} secs", duration)?;
//     Ok(())
// }

// fn solve(input: &mut dyn BufRead, output: &mut dyn Write) -> io::Result<()> {
//     // Implement the solution logic here
//     Ok(())
// }.

fn main() -> () {
    // let mut console_buffer = String::new();
    // std::io::stdin().read_line(&mut console_buffer).ok();

    // print to console
    // println!("{}", console_buffer);

    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35
-1,0.20",
            args[0]
        );
        std::process::exit(1);
    }
    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];
    render(&mut pixels, bounds, upper_left, lower_right);
    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");

    ()
}
// use std::vec::Vec;

// fn build_vector() -> Vec<i16> {
//     let mut v: Vec<i16> = Vec::<i16>::new();
//     v.push(10i16);
//     v.push(20i16);
//     v
// }
// //autotype inference in rust
// fn build_vector2() -> Vec<i16> {
//     let mut v = Vec::new();
//     v.push(10);
//     v.push(20);
//     v
// }

// fn build_mandlebrot() -> Vec<Vec<i16>> {
//     let mut v = Vec::new();
//     v.push(build_vector());
//     v.push(build_vector2());
//     v
// }
