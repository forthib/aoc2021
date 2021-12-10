fn main() {
    part1("example1", read_example1());
    part1("input   ", read_input());
    part2("example1", read_example1());
    part2("input   ", read_input());
}

fn part1(title: &str, lines: Vec<String>) {
    let score: u32 = lines.iter().map(|line| get_syntax_error_score(line)).sum();
    println!("Part 1 - {} -> {}", title, score);
}

fn part2(title: &str, lines: Vec<String>) {
    let lines = lines
        .iter()
        .filter(|line| get_syntax_error_score(line) == 0)
        .map(|s| s.clone())
        .collect::<Vec<String>>();
    let mut scores = lines.iter().map(|line| get_autocomplete_score(line)).collect::<Vec<u64>>();
    scores.sort();
    println!("Part 2 - {} -> {}", title, scores[scores.len() / 2]);
}

fn get_syntax_error_score(line: &String) -> u32 {
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        if is_opening_char(c) {
            stack.push(c);
        } else {
            if stack.is_empty() || c != get_closing_char(stack.pop().unwrap()) {
                return get_error_score(c);
            }
        }
    }
    return 0;
}

fn get_autocomplete_score(line: &String) -> u64 {
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        if is_opening_char(c) {
            stack.push(c);
        } else {
            stack.pop();
        }
    }
    
    let mut score: u64 = 0;
    for c in stack.iter().rev().map(|&c| get_closing_char(c)) {
        score *= 5;
        score += get_complete_score(c);
    }
    return score;
}

fn is_opening_char(c: char) -> bool {
    match c {
        '(' => true,
        '[' => true,
        '{' => true,
        '<' => true,
        _ => false,
    }
}

fn get_closing_char(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => ' ',
    }
}

fn get_error_score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn get_complete_score(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn read_example1() -> Vec<String> {
    return str_to_lines(
        r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#,
    );
}

fn read_input() -> Vec<String> {
    return str_to_lines(
        &std::fs::read_to_string("assets/day10.txt").expect("Unable to read file"),
    );
}

fn str_to_lines(s: &str) -> Vec<String> {
    return s
        .trim()
        .lines()
        .map(|line| line.into())
        .collect::<Vec<String>>();
}
