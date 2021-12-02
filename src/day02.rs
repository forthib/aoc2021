fn main() {
    part1("example1", &read_example1());
    part1("input   ", &read_input());
    part2("example1", &read_example1());
    part2("input   ", &read_input());
}

struct Instr {
    key: String,
    step: i64,
}

fn part1(title: &str, instructions: &Vec<Instr>) {
    let mut pos = 0;
    let mut depth = 0;
    for instr in instructions {
        match instr.key.as_str() {
            "forward" => pos += instr.step,
            "down" => depth += instr.step,
            "up" => depth -= instr.step,
            _ => println!("unknown instruction"),
        }
    }
    println!("Part 1 - {} -> {}", title, pos * depth);
}

fn part2(title: &str, instructions: &Vec<Instr>) {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for instr in instructions {
        match instr.key.as_str() {
            "forward" => {
                pos += instr.step;
                depth += instr.step * aim;
            }
            "down" => aim += instr.step,
            "up" => aim -= instr.step,
            _ => println!("unknown instruction"),
        }
    }
    println!("Part 2 - {} -> {}", title, pos * depth);
}

fn read_example1() -> Vec<Instr> {
    return str_to_instr(
        r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#,
    );
}

fn read_input() -> Vec<Instr> {
    return str_to_instr(
        &std::fs::read_to_string("assets/day02.txt").expect("Unable to read file"),
    );
}

fn str_to_instr(s: &str) -> Vec<Instr> {
    return s
        .lines()
        .map(|line| line_to_instr(line))
        .collect::<Vec<_>>();
}

fn line_to_instr(s: &str) -> Instr {
    let words = s.split_whitespace().collect::<Vec<&str>>();
    let key = String::from(words[0]);
    let step = words[1].parse::<i64>().unwrap();
    return Instr {
        key: key,
        step: step,
    };
}
