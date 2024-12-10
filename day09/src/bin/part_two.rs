use day09::Files;
use util::read_file_to_string;

fn main() {
    let input = read_file_to_string("input/09.txt").unwrap();
    let mut disk = Files::parse(&input);
    disk.defragment();
    let solution = disk.checksum();

    println!("** Solution: {solution} **");
}
