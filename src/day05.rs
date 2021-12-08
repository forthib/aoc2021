use std::cmp;

fn main() {
    part1("example1", read_example1());
    part1("input   ", read_input());
    part2("example1", read_example1());
    part2("input   ", read_input());
}

fn part1(title: &str, lines: Vec<Line>) {
    let map = build_map(&lines, false);
    let count = count_overlaps(map);
    println!("Part 1 - {} -> {}", title, count);
}

fn part2(title: &str, lines: Vec<Line>) {
    let map = build_map(&lines, true);
    let count = count_overlaps(map);
    println!("Part 2 - {} -> {}", title, count);
}

struct Point {
    i: usize,
    j: usize,
}

struct Line {
    a: Point,
    b: Point,
}

struct Map {
    ni: usize,
    values: Vec<usize>,
}

fn init_map(lines: &Vec<Line>) -> Map {
    let mut ni: usize = 0;
    let mut nj: usize = 0;
    for line in lines {
        ni = cmp::max(ni, line.a.i + 1);
        ni = cmp::max(ni, line.b.i + 1);
        nj = cmp::max(nj, line.a.j + 1);
        nj = cmp::max(nj, line.b.j + 1);
    }
    return Map {
        ni: ni,
        values: vec![0usize; (ni * nj) as usize],
    };
}

fn build_map(lines: &Vec<Line>, with_diagonals: bool) -> Map {
    let mut map = init_map(lines);

    for line in lines {
        if line.a.i == line.b.i {
            let first_j = cmp::min(line.a.j, line.b.j);
            let last_j = cmp::max(line.a.j, line.b.j) + 1;
            for j in first_j..last_j {
                map.values[j * map.ni + line.a.i] += 1;
            }
        } else if line.a.j == line.b.j {
            let first_i = cmp::min(line.a.i, line.b.i);
            let last_i = cmp::max(line.a.i, line.b.i) + 1;
            for i in first_i..last_i {
                map.values[line.a.j * map.ni + i] += 1;
            }
        } else if with_diagonals {
            let inc_i = if line.b.i > line.a.i { 1isize } else { -1isize };
            let inc_j = if line.b.j > line.a.j { 1isize } else { -1isize };
            let first_i = line.a.i as isize;
            let first_j = line.a.j as isize;
            let last_i = line.b.i as isize + inc_i;
            let last_j = line.b.j as isize + inc_j;
            let mut i = first_i;
            let mut j = first_j;
            while i != last_i && j != last_j {
                map.values[(j * map.ni as isize + i) as usize] += 1;
                i += inc_i;
                j += inc_j;
            }
        }
    }

    return map;
}

fn count_overlaps(map: Map) -> usize {
    return map.values.iter().filter(|&i| *i >= 2).count();
}

fn read_example1() -> Vec<Line> {
    return str_to_lines(
        r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#,
    );
}

fn read_input() -> Vec<Line> {
    return str_to_lines(
        &std::fs::read_to_string("assets/day05.txt").expect("Unable to read file"),
    );
}

fn str_to_lines(s: &str) -> Vec<Line> {
    return s.lines().map(|s| str_to_line(s)).collect::<Vec<Line>>();
}

fn str_to_line(s: &str) -> Line {
    let mut iter = s.split(" -> ");
    let a = str_to_point(iter.next().unwrap());
    let b = str_to_point(iter.next().unwrap());
    return Line { a: a, b: b };
}

fn str_to_point(s: &str) -> Point {
    let mut iter = s.split(",").map(|i| i.parse::<usize>().unwrap());
    let i = iter.next().unwrap();
    let j = iter.next().unwrap();
    return Point { i: i, j: j };
}
