fn main() {
    part1("example1", read_example1());
    part1("input   ", read_input());
    part2("example1", read_example1());
    part2("input   ", read_input());
}

fn part1(title: &str, mut grid: Grid) {
    let mut sum = 0;
    for _ in 0..100 {
        sum += step(&mut grid);
    }
    println!("Part 1 - {} -> {}", title, sum);
}

fn part2(title: &str, mut grid: Grid) {
    let mut n_steps = 0;
    loop {
        if grid.values.iter().filter(|&i| *i == 0).count() == 100 {
            break;
        }
        step(&mut grid);
        n_steps += 1;
    }
    println!("Part 2 - {} -> {}", title, n_steps);
}

fn step(grid: &mut Grid) -> usize {
    let mut flash_stack: Vec<(usize, usize)> = Vec::new();
    for j in 0..10 {
        for i in 0..10 {
            if grid.increment(i, j) > 9 {
                flash_stack.push((i, j));
            }
        }
    }

    let mut flash_map = Grid {
        values: vec![0; 100],
    };

    while flash_stack.len() > 0 {
        let (i, j) = flash_stack.pop().unwrap();
        if flash_map.get(i, j) > 0 {
            continue;
        }
        flash_map.set(i, j, 1);
        for (i, j) in grid.get_neighbor_indexes(i, j) {
            if grid.increment(i, j) > 9 && flash_map.get(i, j) == 0 {
                flash_stack.push((i, j));
            }
        }
    }

    for j in 0..10 {
        for i in 0..10 {
            if grid.get(i, j) > 9 {
                grid.set(i, j, 0)
            }
        }
    }

    return flash_map.values.iter().filter(|&i| *i > 0).count();
}

struct Grid {
    values: Vec<u32>,
}

impl Grid {
    fn index(&self, i: usize, j: usize) -> usize {
        return j * 10 + i;
    }
    fn get(&self, i: usize, j: usize) -> u32 {
        return self.values[self.index(i, j)];
    }
    fn set(&mut self, i: usize, j: usize, value: u32) {
        let index = self.index(i, j);
        self.values[index] = value;
    }
    fn increment(&mut self, i: usize, j: usize) -> u32 {
        let index = self.index(i, j);
        self.values[index] += 1;
        return self.get(i, j);
    }
    fn get_neighbor_indexes(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let i = i as isize;
        let j = j as isize;

        let mut indexes = Vec::new();
        for di in -1..2 {
            for dj in -1..2 {
                if di != 0 || dj != 0 {
                    let i = i + di;
                    let j = j + dj;
                    if i >= 0 && i < 10 && j >= 0 && j < 10 {
                        indexes.push((i as usize, j as usize));
                    }
                }
            }
        }
        return indexes;
    }
}

fn read_example1() -> Grid {
    return str_to_lines(
        r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#,
    );
}

fn read_input() -> Grid {
    return str_to_lines(
        &std::fs::read_to_string("assets/day11.txt").expect("Unable to read file"),
    );
}

fn str_to_lines(s: &str) -> Grid {
    let values = s
        .trim()
        .lines()
        .map(|line| line.as_bytes())
        .flatten()
        .map(|&i| i as u32 - 48)
        .collect::<Vec<u32>>();
    return Grid { values: values };
}
