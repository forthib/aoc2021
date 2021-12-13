use std::cmp;

fn main() {
    part1("example1", read_example1());
    part1("input   ", read_input());
    part2("input   ", read_example1());
    part2("input   ", read_input());
}

fn part1(title: &str, input: Input) {
    let grid = init_grid(&input.points);
    let grid = fold_once(&grid, input.instructions[0]);
    let count = grid.values.iter().filter(|&b| *b).count();
    println!("Part 1 - {} -> {}", title, count);
}

fn part2(title: &str, input: Input) {
    let mut grid = init_grid(&input.points);
    for instr in input.instructions {
        grid = fold_once(&grid, instr);
    }
    println!("Part 2 - {}", title);
    print(&grid);
}

#[derive(Clone)]
struct Grid {
    nx: usize,
    ny: usize,
    values: Vec<bool>,
}

impl Grid {
    fn new(nx: usize, ny: usize) -> Grid {
        return Grid {
            nx: nx,
            ny: ny,
            values: vec![false; nx * ny],
        };
    }
    fn get(&self, i: usize, j: usize) -> bool {
        return self.values[j * self.nx + i];
    }
    fn set(&mut self, i: usize, j: usize, value: bool) {
        self.values[j * self.nx + i] = value;
    }
}

fn init_grid(points: &Vec<(usize, usize)>) -> Grid {
    let nx = points.iter().map(|p| p.0).max().unwrap() + 1;
    let ny = points.iter().map(|p| p.1).max().unwrap() + 1;
    let mut grid = Grid::new(nx, ny);
    for (i, j) in points {
        grid.set(*i, *j, true);
    }
    return grid;
}

fn fold_once(grid: &Grid, instr: (char, usize)) -> Grid {
    match instr.0 {
        'x' => fold_once_x(grid, instr.1),
        'y' => fold_once_y(grid, instr.1),
        _ => grid.clone(),
    }
}

fn fold_once_x(grid: &Grid, axis_pos: usize) -> Grid {
    let nx = cmp::max(axis_pos, grid.nx - axis_pos - 1);
    let mut result = Grid::new(nx, grid.ny);

    for j in 0..grid.ny {
        for i in 0..nx {
            let value1 = grid.get(i, j);
            let value2 = grid.get(grid.nx - i - 1, j);
            result.set(i, j, value1 || value2);
        }
    }

    return result;
}

fn fold_once_y(grid: &Grid, axis_pos: usize) -> Grid {
    let ny = cmp::max(axis_pos, grid.ny - axis_pos - 1);
    let mut result = Grid::new(grid.nx, ny);

    for j in 0..ny {
        for i in 0..grid.nx {
            let value1 = grid.get(i, j);
            let value2 = grid.get(i, grid.ny - j - 1);
            result.set(i, j, value1 || value2);
        }
    }

    return result;
}

fn print(grid: &Grid) {
    for j in 0..grid.ny {
        for i in 0..grid.nx {
            if grid.get(i, j) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn read_example1() -> Input {
    return str_to_input(
        r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#,
    );
}

fn read_input() -> Input {
    return str_to_input(
        &std::fs::read_to_string("assets/day13.txt").expect("Unable to read file"),
    );
}

struct Input {
    points: Vec<(usize, usize)>,
    instructions: Vec<(char, usize)>,
}

fn str_to_input(s: &str) -> Input {
    let points = s
        .trim()
        .lines()
        .filter(|line| line.len() > 0 && !line.starts_with("fold"))
        .map(|line| str_to_point(line))
        .collect::<_>();
    let instructions = s
        .trim()
        .lines()
        .filter(|line| line.starts_with("fold"))
        .map(|line| str_to_instruction(line))
        .collect::<_>();
    return Input {
        points: points,
        instructions: instructions,
    };
}

fn str_to_point(s: &str) -> (usize, usize) {
    let mut iter = s.split(',').map(|i| i.parse::<usize>().unwrap());
    let x = iter.next().unwrap();
    let y = iter.next().unwrap();
    return (x, y);
}

fn str_to_instruction(s: &str) -> (char, usize) {
    let mut iter = s[11..].split('=');
    let axis = iter.next().map(|i| i.chars().next().unwrap()).unwrap();
    let value = iter.next().map(|i| i.parse::<usize>().unwrap()).unwrap();
    return (axis, value);
}
