static FILENAME: &str = "input/3/data";

fn main() {
    let data = read_input(FILENAME);
    println!("part one: {}", part_one(&data));
    println!("part two: {}", part_two(&data));
}

fn part_one(data: &str) -> usize {
    let chars: Vec<Vec<char>> = data.split('\n').map(|row| row.chars().collect()).collect();

    let length = chars[0].len();

    let mut gamma_v: Vec<char> = Vec::new();
    let mut epsilon_v: Vec<char> = Vec::new();

    for x in 0..length {
        let mut count = 0;
        for y in 0..chars.len() {
            count += if chars[y][x] == '1' { 1 } else { 0 };
        }
        let val = if count > chars.len() / 2 { '1' } else { '0' };
        gamma_v.push(val);
        epsilon_v.push(if val == '0' { '1' } else { '0' });
    }

    let gamma = bin_vec_to_usize(gamma_v);
    let epsilon = bin_vec_to_usize(epsilon_v);

    gamma * epsilon
}

fn part_two(data: &str) -> usize {
    let chars: Vec<Vec<char>> = data.split('\n').map(|row| row.chars().collect()).collect();
    let ox_gen_rating = rating(chars.clone(), '1');
    let scrubber_rating = rating(chars.clone(), '0');
    ox_gen_rating * scrubber_rating
}

fn rating(chars: Vec<Vec<char>>, sig_bit: char) -> usize {
    let mut rating_v: Vec<Vec<char>> = chars.to_vec();

    let mut n = 0;
    while rating_v.len() > 1 {
        rating_v = wash(rating_v, n, sig_bit);
        n += 1;
    }

    bin_vec_to_usize(rating_v[0].clone())
}

fn wash(vec: Vec<Vec<char>>, pos: usize, sig_bit: char) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = Vec::new();
    let mut count = 0;

    for n in 0..vec.len() {
        count += if vec[n][pos] == sig_bit { 1 } else { 0 };
    }

    let significant;

    if count * 2 == vec.len() {
        significant = sig_bit;
    } else {
        significant = if count > vec.len() / 2 { '1' } else { '0' };
    }

    for n in 0..vec.len() {
        if vec[n][pos] == significant {
            res.push(vec[n].to_vec());
        }
    }

    res
}

fn bin_vec_to_usize(v: Vec<char>) -> usize {
    let bin: String = v.into_iter().collect();
    usize::from_str_radix(&bin, 2).unwrap()
}

fn read_input(filename: &str) -> String {
    let mut data = std::fs::read_to_string(filename).expect("could not read file");
    data.truncate(data.len() - 1);
    data
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = super::read_input(super::FILENAME);
        assert_eq!(4118544, super::part_one(&data));
    }

    #[test]
    fn test_part_two() {
        let data = super::read_input(super::FILENAME);
        assert_eq!(3832770, super::part_two(&data));
    }
}
