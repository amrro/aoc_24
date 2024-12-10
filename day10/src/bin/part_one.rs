use day10::TopoMap;
use util::read_file_to_string;

fn main() {
    let input = read_file_to_string("input/10.txt").unwrap();
    let map = TopoMap::from(input.as_str());
    let score = map.total_score();

    println!("* Solution: {score} *");
}
