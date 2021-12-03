fn main() {
    part1("example1", &read_example1(), 5);
    part1("input   ", &read_input(), 12);
    part2("example1", &read_example1(), 5);
    part2("input   ", &read_input(), 12);
}

fn part1(title: &str, numbers: &Vec<u32>, n_digits: u8) {
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..n_digits {
        if get_most_common_bit_at(numbers, i) {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }
    println!("Part 1 - {} -> {}", title, gamma * epsilon);
}

fn part2(title: &str, numbers: &Vec<u32>, n_digits: u8) {
    let oxygen = filter_most_common_value_all(numbers.to_vec(), n_digits, true);
    let co2 = filter_most_common_value_all(numbers.to_vec(), n_digits, false);
    println!("Part 2 - {} -> {}", title, oxygen * co2);
}

fn filter_most_common_value_all(mut numbers: Vec<u32>, n_digits: u8, criteria: bool) -> u32 {
    for i in 0..n_digits {
        if numbers.len() == 1 {
            break;
        }
        numbers = filter_most_common_value_once(&numbers, n_digits - i - 1, criteria);
    }
    return numbers[0];
}

fn filter_most_common_value_once(numbers: &Vec<u32>, i: u8, criteria: bool) -> Vec<u32> {
    let common_bit = get_most_common_bit_at(numbers, i) ^ !criteria;
    return numbers
        .iter()
        .map(|&n| n)
        .filter(|&n| get_bit_at(n, i) == common_bit)
        .collect::<Vec<_>>();
}

fn get_most_common_bit_at(numbers: &Vec<u32>, i: u8) -> bool {
    let count_0 = numbers.iter().filter(|&n| !get_bit_at(*n, i)).count();
    let count_1 = numbers.iter().filter(|&n| get_bit_at(*n, i)).count();
    return count_1 >= count_0;
}

fn get_bit_at(n: u32, i: u8) -> bool {
    n & (1 << i) != 0
}

fn read_example1() -> Vec<u32> {
    return str_to_decimals(
        r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#,
    );
}

fn read_input() -> Vec<u32> {
    return str_to_decimals(
        &std::fs::read_to_string("assets/day03.txt").expect("Unable to read file"),
    );
}

fn str_to_decimals(s: &str) -> Vec<u32> {
    return s
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect::<Vec<u32>>();
}
