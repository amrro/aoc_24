use day09::DiskMap;
use util::read_file_to_string;

fn main() {
    let input = read_file_to_string("input/09.txt").unwrap();
    let mut disk = DiskMap::parse(&input);
    disk.fragment();
    let solution = disk.checksum();

    println!("* Solution: {solution} *");
}
