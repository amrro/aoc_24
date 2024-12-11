use day09::Disk;
use util::read_file_to_string;

fn main() {
    let input = read_file_to_string("input/09.txt").unwrap();
    let mut disk = Disk::parse(&input);
    disk.defragment();
    let solution = disk.checksum();

    println!("* Solution: {solution} *");
}