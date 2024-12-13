use day12::Garden;
use util::read_file_to_string;

fn main() {
    let input = read_file_to_string("input/12.txt").unwrap();
    let garden = Garden::from(input.as_str());

    let total_price = garden.total_price(true);

    println!("* Solution: {total_price} *");
}
