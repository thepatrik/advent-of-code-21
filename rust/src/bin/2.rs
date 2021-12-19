static FILENAME: &str = "input/2/data";

type Operations<'a> = Vec<(&'a str, isize)>;

fn main() {
    let data = std::fs::read_to_string(FILENAME).expect("could not read file");
    println!("Part one: {:?}", part_one(&data));
    println!("Part two: {:?}", part_two(&data));
}

fn part_one(data: &str) -> isize {
    let ops = parse_operations(&data);

    let mut forward: isize = 0;
    let mut depth: isize = 0;
    for x in 0..ops.len() {
        match ops[x].0 {
            "forward" => {
                forward += ops[x].1;
            }
            "up" => depth -= ops[x].1,
            _ => depth += ops[x].1,
        }
    }

    forward * depth
}

fn part_two(data: &str) -> isize {
    let ops = parse_operations(&data);

    let mut forward: isize = 0;
    let mut depth: isize = 0;
    let mut aim: isize = 0;
    for x in 0..ops.len() {
        match ops[x].0 {
            "forward" => {
                forward += ops[x].1;
                depth += ops[x].1 * aim;
            }
            "up" => aim -= ops[x].1,
            _ => aim += ops[x].1,
        }
    }

    forward * depth
}

fn parse_operations(data: &str) -> Operations {
    let mut ops = Vec::new();
    for line in data.lines() {
        let split = line.split(" ").collect::<Vec<&str>>();
        ops.push((split[0], split[1].parse::<isize>().unwrap()));
    }
    ops
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(1714950, super::part_one(&data));
    }
    #[test]
    fn test_part_two() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(1281977850, super::part_two(&data));
    }
}
