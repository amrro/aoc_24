use std::io::BufRead;

use day06::Map;
use util::read_file;

fn main() {
    let input = read_file("input/06.txt")
        .lines()
        .map_while(Result::ok)
        .map(|l| l.chars().collect())
        .collect();

    let mut map = Map::new(input);
    map.walk();
    let output = map.count_steps();
    println!("* Solution: {output} *");
}
