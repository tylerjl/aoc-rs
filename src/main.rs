use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

mod y2020;
mod y2022;

#[derive(StructOpt)]
#[structopt(about = "Advent of Code solutions")]
struct Cli {
    /// Year for desired solution
    year: usize,
    /// Day for desired solution
    day: usize,
    /// Part for desired solution (either 'a' or 'b')
    part: char,
    /// File path to problem input
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let args = Cli::from_args();

    match (args.year, args.day, args.part) {
        (2020, 1, 'a') => {
            let input = fs::read_to_string(args.input).unwrap();
            println!("{:?}", y2020::day01::part_a(&input));
        }
        (2020, 1, 'b') => {
            let input = fs::read_to_string(args.input).unwrap();
            println!("{:?}", y2020::day01::part_b(&input));
        }
        (2022, 1, 'a') => {
            let input = fs::read_to_string(args.input).unwrap();
            println!("{:?}", y2022::day01::part_a(input));
        }
        (2022, 1, 'b') => {
            let input = fs::read_to_string(args.input).unwrap();
            println!("{:?}", y2022::day01::part_b(input));
        }
        (year, day, part) => {
            println!("Can't handle {} day {} part {}", year, day, part)
        }
    }
}
