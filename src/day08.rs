use std::collections::HashSet;

fn main() {
    part1("example2", read_example2());
    part1("input   ", read_input());
    part2("example1", read_example1());
    part2("example2", read_example2());
    part2("input   ", read_input());
}

fn part1(title: &str, lines: Vec<Line>) {
    let n1 = count_output_digits_with_length(&lines, 2);
    let n4 = count_output_digits_with_length(&lines, 4);
    let n7 = count_output_digits_with_length(&lines, 3);
    let n8 = count_output_digits_with_length(&lines, 7);
    println!("Part 1 - {} -> {}", title, n1 + n4 + n7 + n8);
}

fn part2(title: &str, lines: Vec<Line>) {
    let n: usize = lines.iter().map(|line| decode(line)).sum();
    println!("Part 2 - {} -> {}", title, n);
}

fn count_output_digits_with_length(lines: &Vec<Line>, length: usize) -> usize {
    return lines
        .iter()
        .map(|line| &line.output)
        .flatten()
        .filter(|digit| digit.len() == length)
        .count();
}

fn decode(line: &Line) -> usize {
    let key = find_key(&line.input);
    let result4 = line
        .output
        .iter()
        .map(|digit| find_value(digit, &key))
        .collect::<Vec<usize>>();

    let mut result: usize = 0;
    for i in 0..4 {
        result *= 10;
        result += result4[i];
    }
    return result;
}

fn find_value(digit: &Digit, key: &Vec<Digit>) -> usize {
    for i in 0..10 {
        if key[i] == *digit {
            return i;
        }
    }
    return 0;
}

fn find_key(input: &Vec<Digit>) -> Vec<Digit> {
    let digits5 = get_digits_with_length(input, 5);
    let digits6 = get_digits_with_length(input, 6);

    let mut key: Vec<Digit> = Vec::new();
    key.resize(10, Digit::new());
    key[1] = find_digit(input, |d| d.len() == 2);
    key[4] = find_digit(input, |d| d.len() == 4);
    key[7] = find_digit(input, |d| d.len() == 3);
    key[8] = find_digit(input, |d| d.len() == 7);
    key[3] = find_digit(&digits5, |d| n_common(d, &key[1]) == 2);
    key[9] = find_digit(&digits6, |d| n_common(d, &key[4]) == 4);
    key[5] = find_digit(&digits5, |d| n_common(d, &key[9]) == 5 && d != &key[3]);
    key[2] = find_digit(&digits5, |d| d != &key[3] && d != &key[5]);
    key[6] = find_digit(&digits6, |d| n_common(d, &key[1]) == 1);
    key[0] = find_digit(&digits6, |d| d != &key[6] && d != &key[9]);
    return key;
}

fn find_digit(digits: &Vec<Digit>, predicate: impl Fn(&Digit) -> bool) -> Digit {
    return digits
        .iter()
        .filter(|&digit| predicate(digit))
        .next()
        .unwrap()
        .clone();
}

fn n_common(lhs: &Digit, rhs: &Digit) -> usize {
    return lhs.intersection(&rhs).count();
}

fn get_digits_with_length(input: &Vec<Digit>, length: usize) -> Vec<Digit> {
    return input
        .iter()
        .filter(|digit| digit.len() == length)
        .map(|d| d.clone())
        .collect::<Vec<_>>();
}

type Digit = HashSet<u8>;

struct Line {
    input: Vec<Digit>,
    output: Vec<Digit>,
}

fn read_example1() -> Vec<Line> {
    return str_to_lines(
        r#"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"#,
    );
}

fn read_example2() -> Vec<Line> {
    return str_to_lines(
        r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#,
    );
}

fn read_input() -> Vec<Line> {
    return str_to_lines(
        &std::fs::read_to_string("assets/day08.txt").expect("Unable to read file"),
    );
}

fn str_to_lines(s: &str) -> Vec<Line> {
    return s.trim().lines().map(|s| str_to_line(s)).collect::<Vec<_>>();
}

fn str_to_line(s: &str) -> Line {
    let mut iter = s.trim().split("|").map(|s| str_to_digits(s));
    let input = iter.next().unwrap();
    let output = iter.next().unwrap();
    return Line {
        input: input,
        output: output,
    };
}

fn str_to_digits(s: &str) -> Vec<Digit> {
    return s
        .trim()
        .split_whitespace()
        .map(|s| str_to_digit(s))
        .collect::<Vec<Digit>>();
}

fn str_to_digit(s: &str) -> Digit {
    return s.as_bytes().to_vec().iter().map(|&i| i).collect::<Digit>();
}
