static FILENAME: &str = "input/1/data";

fn part_one(nums: &Vec<isize>) -> isize {
    let mut count = 0;
    for n in 1..nums.len() {
        if nums[n] > nums[n - 1] {
            count += 1;
        }
    }
    count
}

fn part_two(nums: &Vec<isize>) -> isize {
    let mut count = 0;
    for n in 3..nums.len() {
        let sum = nums[n - 3] + nums[n - 2] + nums[n - 1];
        let sum_next = nums[n - 2] + nums[n - 1] + nums[n];
        if sum_next > sum {
            count += 1;
        }
    }
    count
}

fn parse(data: &str) -> Option<Vec<isize>> {
    data.trim().split("\n").map(|s| s.parse().ok()).collect()
}

fn main() {
    let data = std::fs::read_to_string(FILENAME).expect("could not read file");
    let nums = parse(&data).unwrap();
    println!("Part one: {:?}", part_one(&nums));
    println!("Part two: {:?}", part_two(&nums));
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        let nums = super::parse(&data).unwrap();
        assert_eq!(1154, super::part_one(&nums));
    }
    #[test]
    fn test_part_two() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        let nums = super::parse(&data).unwrap();
        assert_eq!(1127, super::part_two(&nums));
    }
}
