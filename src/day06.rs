use std::collections::HashMap;

fn main() {
    run("Part1 - example1", read_example1(), 18);
    run("Part1 - example1", read_example1(), 80);
    run("Part1 - input   ", read_input(), 80);
    run("Part2 - example1", read_example1(), 256);
    run("Part2 - input   ", read_input(), 256);
}

fn run(title: &str, fishes: Vec<usize>, n_days: usize) {
    let n = count_all(fishes, n_days);
    println!("{} - {} days -> {}", title, n_days, n);
}

fn count_all(fishes: Vec<usize>, n_days: usize) -> usize {
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();

    let mut count: usize = fishes.len();
    for fish in fishes {
        count += count_one(fish, n_days, &mut map);
    }
    return count;
}

fn count_one(n_start: usize, n_days: usize, map: &mut HashMap<(usize, usize), usize>) -> usize {
    if map.contains_key(&(n_start, n_days)) {
        return map.get(&(n_start, n_days)).map(|&i| i).unwrap_or(0);
    }

    if n_days < n_start + 1 {
        return 0;
    }

    let n = (n_days - n_start - 1) / 7 + 1;
    let mut count = n;
    for i in 0..n {
        count += count_one(8, n_days - n_start - 1 - i * 7, map);
    }

    map.insert((n_start, n_days), count);

    return count;
}

fn read_example1() -> Vec<usize> {
    return str_to_input(r#"3,4,3,1,2"#);
}

fn read_input() -> Vec<usize> {
    return str_to_input(
        &std::fs::read_to_string("assets/day06.txt").expect("Unable to read file"),
    );
}

fn str_to_input(s: &str) -> Vec<usize> {
    return s
        .trim()
        .split(",")
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
}
