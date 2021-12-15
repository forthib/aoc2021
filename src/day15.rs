use pathfinding::prelude::dijkstra;

fn main() {
    part1("example1", read_example1());
    part1("input   ", read_input());
    part2("example1", read_example1());
    part2("input   ", read_input());
}

fn part1(title: &str, grid: Grid) {
    println!("Part 1 - {} -> {}", title, run(grid, 1));
}

fn part2(title: &str, grid: Grid) {
    println!("Part 2 - {} -> {}", title, run(grid, 5));
}

fn run(grid: Grid, n_repeats: usize) -> usize {
    let start = (0, 0);
    let goal = (grid.n * n_repeats - 1, grid.n * n_repeats - 1);
    let result = dijkstra(
        &start,
        |p| get_successors(&grid, &p, n_repeats),
        |p| *p == goal,
    )
    .unwrap();
    return result.1;
}

struct Grid {
    n: usize,
    values: Vec<usize>,
}

impl Grid {
    fn get(&self, i: usize, j: usize) -> usize {
        let i_local = i % self.n;
        let j_local = j % self.n;
        let n_incr = (i / self.n) + (j / self.n);
        let value = self.values[j_local * self.n + i_local];
        let value = ((value - 1 + n_incr) % 9) + 1;
        return value;
    }
}

type Pos = (usize, usize);

fn get_successors(grid: &Grid, pos: &Pos, n_repeats: usize) -> Vec<(Pos, usize)> {
    let (i, j) = *pos;

    let mut successors = Vec::new();
    if i > 0 {
        successors.push(((i - 1, j), grid.get(i - 1, j)))
    }
    if i < grid.n * n_repeats - 1 {
        successors.push(((i + 1, j), grid.get(i + 1, j)))
    }
    if j > 0 {
        successors.push(((i, j - 1), grid.get(i, j - 1)))
    }
    if j < grid.n * n_repeats - 1 {
        successors.push(((i, j + 1), grid.get(i, j + 1)))
    }
    return successors;
}

fn read_example1() -> Grid {
    return str_to_grid(
        r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#,
    );
}

fn read_input() -> Grid {
    return str_to_grid(&std::fs::read_to_string("assets/day15.txt").expect("Unable to read file"));
}

fn str_to_grid(s: &str) -> Grid {
    let n = s.trim().lines().next().unwrap().len();
    let values = s
        .trim()
        .lines()
        .map(|line| line.as_bytes())
        .flatten()
        .map(|&i| i as usize - 48)
        .collect::<Vec<usize>>();
    return Grid {
        n: n,
        values: values,
    };
}
