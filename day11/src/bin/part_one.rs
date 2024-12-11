use day11::Stones;
use util::read_file_to_string;

fn main() {
    let input = read_file_to_string("input/11.txt").unwrap();
    let mut stones = Stones::new(input.as_str());
    let solution = stones.repeat(75);

    println!("* Solution: {solution} *");
}
