fn main() {
    println!("part one: {}", part_one(FISH.to_vec()));
    println!("part two: {}", part_two(FISH.to_vec()));
}

fn part_one(fish: Vec<usize>) -> usize {
    count_fish(fish, 80)
}

fn part_two(fish: Vec<usize>) -> usize {
    count_fish(fish, 256)
}

fn count_fish(ancestors: Vec<usize>, days: usize) -> usize {
    let mut fishes: [usize; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

    for ancestor in ancestors {
        fishes[ancestor] += 1;
    }

    for _ in 0..days {
        let f_0 = fishes[0];

        for n in 1..9 {
            fishes[n - 1] = fishes[n];
        }

        fishes[6] += f_0;
        fishes[8] = f_0;
    }

    fishes.iter().sum()
}

mod tests {
    #[test]
    fn test_part_one() {
        assert_eq!(349549, super::part_one(super::FISH.to_vec()));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(1589590444365, super::part_two(super::FISH.to_vec()));
    }
}

const FISH: [usize; 300] = [
    1, 4, 2, 4, 5, 3, 5, 2, 2, 5, 2, 1, 2, 4, 5, 2, 3, 5, 4, 3, 3, 1, 2, 3, 2, 1, 4, 4, 2, 1, 1, 4,
    1, 4, 4, 4, 1, 4, 2, 4, 3, 3, 3, 3, 1, 1, 5, 4, 2, 5, 2, 4, 2, 2, 3, 1, 2, 5, 2, 4, 1, 5, 3, 5,
    1, 4, 5, 3, 1, 4, 5, 2, 4, 5, 3, 1, 2, 5, 1, 2, 2, 1, 5, 5, 1, 1, 1, 4, 2, 5, 4, 3, 3, 1, 3, 4,
    1, 1, 2, 2, 2, 5, 4, 4, 3, 2, 1, 1, 1, 1, 2, 5, 1, 3, 2, 1, 4, 4, 2, 1, 4, 5, 2, 5, 5, 3, 3, 1,
    3, 2, 2, 3, 4, 1, 3, 1, 5, 4, 2, 5, 2, 4, 1, 5, 1, 4, 5, 1, 2, 4, 4, 1, 4, 1, 4, 4, 2, 2, 5, 4,
    1, 3, 1, 3, 3, 1, 5, 1, 5, 5, 5, 1, 3, 1, 2, 1, 4, 5, 4, 4, 1, 3, 3, 1, 4, 1, 2, 1, 3, 2, 1, 5,
    5, 3, 3, 1, 3, 5, 1, 5, 3, 5, 3, 1, 1, 1, 1, 4, 4, 3, 5, 5, 1, 1, 2, 2, 5, 5, 3, 2, 5, 2, 3, 4,
    4, 1, 1, 2, 2, 4, 3, 5, 5, 1, 1, 5, 4, 3, 1, 3, 1, 2, 4, 4, 4, 4, 1, 4, 3, 4, 1, 3, 5, 5, 5, 1,
    3, 5, 4, 3, 1, 3, 5, 4, 4, 3, 4, 2, 1, 1, 3, 1, 1, 2, 4, 1, 4, 1, 1, 1, 5, 5, 1, 3, 4, 1, 1, 5,
    4, 4, 2, 2, 1, 3, 4, 4, 2, 2, 2, 3,
];
