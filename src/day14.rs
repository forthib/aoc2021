use std::collections::HashMap;

fn main() {
    part1("example1", read_example1());
    part1("input   ", read_input());
    part2("example1", read_example1());
    part2("input   ", read_input());
}

fn part1(title: &str, input: Input) {
    println!("Part 1 - {} -> {}", title, run(input, 10));
}

fn part2(title: &str, input: Input) {
    println!("Part 2 - {} -> {}", title, run(input, 40));
}

fn run(input: Input, n: usize) -> usize {
    let mut map = init_map(&input);
    for _ in 0..n {
        map = step(&input, map);
    }
    let counts = count_elements(&input, &map);
    return counts[counts.len() - 1] - counts[0];
}

fn init_map(input: &Input) -> HashMap<(char, char), usize> {
    let mut map = HashMap::new();

    for (a, b, _) in &input.rules {
        map.insert((*a, *b), 0);
    }
    for ab in input.polymer.windows(2) {
        let a = ab[0];
        let b = ab[1];
        add(&mut map, a, b, 1);
    }

    return map;
}

fn step(input: &Input, map: HashMap<(char, char), usize>) -> HashMap<(char, char), usize> {
    let mut next_map = HashMap::new();
    for (a, b) in map.keys() {
        let value = *map.get(&(*a, *b)).unwrap();
        let c = input
            .rules
            .iter()
            .filter(|(c, d, _)| a == c && b == d)
            .map(|(_, _, e)| *e)
            .next()
            .unwrap();
        add(&mut next_map, *a, c, value);
        add(&mut next_map, c, *b, value);
    }
    return next_map;
}

fn add(map: &mut HashMap<(char, char), usize>, a: char, b: char, value: usize) {
    map.insert((a, b), map.get(&(a, b)).unwrap_or(&0usize) + value);
}

fn count_elements(input: &Input, map: &HashMap<(char, char), usize>) -> Vec<usize> {
    let mut counts = get_elements(&input)
        .iter()
        .map(|c| count_element(&input, &map, *c))
        .collect::<Vec<_>>();
    counts.sort();
    return counts;
}

fn count_element(input: &Input, map: &HashMap<(char, char), usize>, c: char) -> usize {
    let mut count = 0;
    for (a, b) in map.keys() {
        if *a == c {
            count += map.get(&(*a, *b)).unwrap();
        }
        if *b == c {
            count += map.get(&(*a, *b)).unwrap();
        }
    }
    count /= 2;

    if c == input.polymer[0] {
        count += 1;
    }
    if c == input.polymer[input.polymer.len() - 1] {
        count += 1;
    }

    return count;
}

fn get_elements(input: &Input) -> Vec<char> {
    let mut elements = Vec::new();
    for (a, b, _) in input.rules.to_vec() {
        elements.push(a);
        elements.push(b);
    }
    elements.sort();
    elements.dedup();
    return elements;
}

fn read_example1() -> Input {
    return str_to_input(
        r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#,
    );
}

fn read_input() -> Input {
    return str_to_input(
        &std::fs::read_to_string("assets/day14.txt").expect("Unable to read file"),
    );
}

struct Input {
    polymer: Vec<char>,
    rules: Vec<(char, char, char)>,
}

fn str_to_input(s: &str) -> Input {
    let polymer = s.trim().lines().next().unwrap();
    let polymer = polymer.chars().collect::<Vec<char>>();
    let rules = s
        .trim()
        .lines()
        .skip(2)
        .map(|s| str_to_rule(s))
        .collect::<Vec<_>>();
    return Input {
        polymer: polymer,
        rules: rules,
    };
}

fn str_to_rule(s: &str) -> (char, char, char) {
    let mut iter = s.split(" -> ");
    let (c1, c2) = iter.next().map(|s| str_to_pair(s)).unwrap();
    let c3 = iter.next().map(|s| s.chars().next().unwrap()).unwrap();
    return (c1, c2, c3);
}

fn str_to_pair(s: &str) -> (char, char) {
    let mut iter = s.chars();
    let c1 = iter.next().unwrap();
    let c2 = iter.next().unwrap();
    return (c1, c2);
}
