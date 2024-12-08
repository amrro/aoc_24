use day08::City;
use util::read_file_to_string;

fn main() {
    let input = read_file_to_string("input/08.txt").unwrap();
    let city = City::from(&input);
    let result = city.get_unique_antinode_count();
    println!("* Solution: {result} *");
}
