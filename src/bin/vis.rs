use itertools::Itertools;
use tools::*;

fn main() {
    let args = std::env::args().collect_vec();
    if args.len() != 3 {
        eprintln!("Usage: {} <input> <output>", args[0]);
        return;
    }
    let s = (1..=2).map(|i|
        std::fs::read_to_string(&args[i]).unwrap_or_else(|_| {
            eprintln!("no such file: {}", args[i]);
            std::process::exit(1);
        })).collect_vec();
    let input = parse_input(&s[0]);
    let output = parse_output(&s[1]);

    let (score, err) = match output {
        Ok(output) => compute_score(&input, &output),
        Err(err) => (0, err),
    };

    println!("Score = {}", score);
    if err.len() > 0 {
        println!("{}", err);
    }
}
