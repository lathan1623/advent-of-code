use std::env;
pub use std::fs::File;
use std::io;
pub use std::io::BufRead;
pub use std::path::Path;

mod year_2025;

trait Problem {
    fn part_one(&self) -> u64;
    fn part_two(&self) -> u64;
}

fn main() {
    let problem : &dyn Problem = match env::args().nth(1).as_deref() {
        Some("2025/1") => &year_2025::problem_one::Input,
        Some("2025/2") => &year_2025::problem_two::Input,
        Some("2025/3") => &year_2025::problem_three::Input,
        _ => panic!("Invalid input"),
    };

    let solution_one = problem.part_one();
    let solution_two = problem.part_two();

    println!("{} \n {}", solution_one, solution_two);
}

fn read_file<P>(filename: P) -> io::Result<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}
