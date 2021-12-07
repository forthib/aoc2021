fn main() {
    part1("example1", read_example1());
    part1("input   ", read_input());
    part2("example1", read_example1());
    part2("input   ", read_input());
}

fn part1(title: &str, positions: Vec<usize>) {
    let max_pos = positions.iter().map(|&i| i).max().unwrap_or(0);
    let cost = (0..max_pos)
        .map(|pos| cost_to_pos_part1(&positions, pos))
        .min()
        .unwrap_or(0);
    println!("Part 1 - {} -> {}", title, cost);
}

fn part2(title: &str, positions: Vec<usize>) {
    let max_pos = positions.iter().map(|&i| i).max().unwrap_or(0);
    let cost = (0..max_pos)
        .map(|pos| cost_to_pos_part2(&positions, pos))
        .min()
        .unwrap_or(0);
    println!("Part 2 - {} -> {}", title, cost);
}

fn cost_to_pos_part1(positions: &Vec<usize>, pos: usize) -> usize {
    return positions.iter().map(|&p| dist(p, pos)).sum();
}

fn cost_to_pos_part2(positions: &Vec<usize>, pos: usize) -> usize {
    return positions.iter().map(|&p| cost(p, pos)).sum();
}

fn dist(pos1: usize, pos2: usize) -> usize {
    return if pos1 < pos2 {
        pos2 - pos1
    } else {
        pos1 - pos2
    };
}

fn cost(pos1: usize, pos2: usize) -> usize {
    let d = dist(pos1, pos2);
    return d * (d + 1) / 2;
}

fn read_example1() -> Vec<usize> {
    return str_to_input(r#"16,1,2,0,4,2,7,1,2,14"#);
}

fn read_input() -> Vec<usize> {
    return str_to_input(
        &std::fs::read_to_string("assets/day07.txt").expect("Unable to read file"),
    );
}

fn str_to_input(s: &str) -> Vec<usize> {
    return s
        .trim()
        .split(",")
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
}
