fn main() {
    part1("example1", &read_example1());
    part1("input   ", &read_input());
    part2("example1", &read_example1());
    part2("input   ", &read_input());
}

fn part1(title: &str, values: &Vec<u64>) {
    let n = values.windows(2).filter(|ij| return ij[0] < ij[1]).count();
    println!("Part 1 - {} -> {}", title, n);
}

fn part2(title: &str, values: &Vec<u64>) {
    let sums = values.windows(3).map(|ijk| return ijk[0] + ijk[1] + ijk[2]).collect::<Vec<u64>>();
    let n = sums.windows(2).filter(|ij| return ij[0] < ij[1]).count();
    println!("Part 2 - {} -> {}", title, n);
}

fn read_example1() -> Vec<u64> {
    return str_to_ints(
        r#"199
200
208
210
200
207
240
269
260
263"#,
    );
}

fn read_input() -> Vec<u64> {
    return str_to_ints(&std::fs::read_to_string("assets/day01.txt").expect("Unable to read file"));
}

fn str_to_ints(s: &str) -> Vec<u64> {
    return s
        .lines()
        .map(|i| i.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
}
