fn main() {
    part1("example1", read_example1());
    part1("input   ", read_input());
    part2("example1", read_example1());
    part2("input   ", read_input());
}

fn part1(title: &str, map: Map) {
    let mut risk: u32 = 0;
    for (i, j) in find_lowest_points(&map) {
        risk += map.get(i, j) + 1;
    }
    println!("Part 1 - {} -> {}", title, risk);
}

fn part2(title: &str, map: Map) {
    let mut basins_sizes = find_basins(&map)
        .iter()
        .map(|basin| basin.len())
        .collect::<Vec<usize>>();
    basins_sizes.sort();
    let score: usize = basins_sizes.iter().rev().take(3).product();
    println!("Part 2 - {} -> {}", title, score);
}

struct Map {
    ni: usize,
    nj: usize,
    values: Vec<u32>,
}

impl Map {
    fn index(&self, i: usize, j: usize) -> usize {
        return j * self.ni + i;
    }
    fn get(&self, i: usize, j: usize) -> u32 {
        return self.values[self.index(i, j)];
    }
}

fn find_basins(map: &Map) -> Vec<Vec<(usize, usize)>> {
    let mut basins = Vec::new();

    let mut explored: Vec<bool> = Vec::new();
    explored.resize(map.values.len(), false);

    for j in 0..map.nj {
        for i in 0..map.ni {
            if !explored[map.index(i, j)] && map.get(i, j) != 9 {
                basins.push(find_basin(map, i, j, &mut explored));
            }
        }
    }

    return basins;
}

fn find_basin(map: &Map, i: usize, j: usize, explored: &mut Vec<bool>) -> Vec<(usize, usize)> {
    let mut basin = Vec::new();

    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push((i, j));
    explored[map.index(i, j)] = true;

    while stack.len() > 0 {
        let (i, j) = stack.pop().unwrap();
        basin.push((i, j));

        for (i, j) in get_neighbor_points(map, i, j) {
            if !explored[map.index(i, j)] && map.get(i, j) != 9 {
                stack.push((i, j));
                explored[map.index(i, j)] = true;
            }
        }
    }

    return basin;
}

fn find_lowest_points(map: &Map) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    for j in 0..map.nj {
        for i in 0..map.ni {
            if is_lowest_point(map, i, j) {
                points.push((i, j));
            }
        }
    }
    return points;
}

fn is_lowest_point(map: &Map, i: usize, j: usize) -> bool {
    let value = map.get(i, j);
    for (i, j) in get_neighbor_points(map, i, j) {
        if value >= map.get(i, j) {
            return false;
        }
    }
    return true;
}

fn get_neighbor_points(map: &Map, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    if i > 0 {
        points.push((i - 1, j))
    }
    if i < map.ni - 1 {
        points.push((i + 1, j))
    }
    if j > 0 {
        points.push((i, j - 1))
    }
    if j < map.nj - 1 {
        points.push((i, j + 1))
    }
    return points;
}

fn read_example1() -> Map {
    return str_to_map(
        r#"2199943210
3987894921
9856789892
8767896789
9899965678"#,
    );
}

fn read_input() -> Map {
    return str_to_map(&std::fs::read_to_string("assets/day09.txt").expect("Unable to read file"));
}

fn str_to_map(s: &str) -> Map {
    let values = s
        .trim()
        .lines()
        .map(|line| line.as_bytes())
        .flatten()
        .map(|&i| i as u32 - 48)
        .collect::<Vec<u32>>();
    let ni = s.trim().lines().next().unwrap().len();
    let nj = values.len() / ni;
    return Map {
        ni: ni,
        nj: nj,
        values: values,
    };
}
