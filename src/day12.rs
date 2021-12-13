use std::collections::HashMap;

fn main() {
    part1("example1", read_example1());
    part1("example2", read_example2());
    part1("example3", read_example3());
    part1("input   ", read_input());
    part2("example1", read_example1());
    part2("example2", read_example2());
    part2("example3", read_example3());
    part2("input   ", read_input());
}

fn part1(title: &str, graph: Graph) {
    let mut map: HashMap<(Graph, String), u64> = HashMap::new();
    let count = count_all_paths_p1(&graph, "start".into(), &mut map);
    println!("Part 1 - {} -> {}", title, count);
}

fn part2(title: &str, graph: Graph) {
    let mut map: HashMap<(Graph, String, Option<String>), u64> = HashMap::new();
    let count = count_all_paths_p2(
        &graph,
        &"start".into(),
        &mut map,
        &Option::None,
        &Vec::new(),
    );
    println!("Part 2 - {} -> {}", title, count);
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Graph {
    verts: Vec<Vert>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Vert {
    id: String,
    big: bool,
    neighbors: Vec<String>,
}

fn count_all_paths_p1(
    graph: &Graph,
    start: String,
    map: &mut HashMap<(Graph, String), u64>,
) -> u64 {
    if start == "end" {
        return 1;
    }

    let count = map.get(&(graph.clone(), start.clone()));
    if count.is_some() {
        return *count.unwrap();
    }

    let mut count: u64 = 0;

    let vert = graph.verts.iter().filter(|v| v.id == start).next().unwrap();
    let graph2 = remove_vertex(&graph, &start);

    let neighbors = vert.neighbors.clone();
    for nid in neighbors {
        if vert.big {
            count += count_all_paths_p1(&graph, nid, map);
        } else {
            count += count_all_paths_p1(&graph2, nid, map);
        }
    }

    map.insert((graph.clone(), start.clone()), count);

    return count;
}

fn count_all_paths_p2(
    graph: &Graph,
    start: &String,
    map: &mut HashMap<(Graph, String, Option<String>), u64>,
    small_cave_id: &Option<String>,
    path: &Vec<String>,
) -> u64 {
    let mut path = path.clone();
    path.push(start.clone());

    if start == "end" {
        if small_cave_id.is_some()
            && path
                .iter()
                .filter(|&id| *id == small_cave_id.clone().unwrap())
                .count()
                <= 1
        {
            return 0;
        }
        return 1;
    }

    let count = map.get(&(graph.clone(), start.clone(), small_cave_id.clone()));
    if count.is_some() {
        return *count.unwrap();
    }

    let mut count: u64 = 0;

    let vert = graph
        .verts
        .iter()
        .filter(|v| v.id == *start)
        .next()
        .unwrap();
    let graph2 = remove_vertex(&graph, &start);

    let neighbors = vert.neighbors.clone();
    for nid in neighbors {
        if vert.big {
            count += count_all_paths_p2(&graph, &nid, map, &small_cave_id, &path);
        } else if vert.id == "start" {
            count += count_all_paths_p2(&graph2, &nid, map, &small_cave_id, &path);
        } else {
            if small_cave_id.is_none() {
                count +=
                    count_all_paths_p2(&graph, &nid, map, &Option::from(vert.id.clone()), &path);
                count += count_all_paths_p2(&graph2, &nid, map, &Option::None, &path);
            } else if small_cave_id.clone().unwrap() == vert.id {
                count += count_all_paths_p2(&graph2, &nid, map, &small_cave_id, &path);
            } else {
                count += count_all_paths_p2(&graph2, &nid, map, &small_cave_id, &path);
            }
        }
    }

    map.insert((graph.clone(), start.clone(), small_cave_id.clone()), count);

    return count;
}

fn remove_vertex(graph: &Graph, id: &String) -> Graph {
    let verts = graph
        .verts
        .iter()
        .filter(|v| v.id != *id)
        .map(|v| remove_neighbor(v, &id))
        .collect::<Vec<Vert>>();

    return Graph { verts: verts };
}

fn remove_neighbor(vert: &Vert, id: &String) -> Vert {
    let mut vert = vert.clone();
    vert.neighbors = vert
        .neighbors
        .iter()
        .filter(|&s| s != id)
        .map(|s| s.clone())
        .collect::<Vec<String>>();
    return vert;
}

fn read_example1() -> Graph {
    return str_to_graph(
        r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#,
    );
}

fn read_example2() -> Graph {
    return str_to_graph(
        r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#,
    );
}

fn read_example3() -> Graph {
    return str_to_graph(
        r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#,
    );
}

fn read_input() -> Graph {
    return str_to_graph(
        &std::fs::read_to_string("assets/day12.txt").expect("Unable to read file"),
    );
}

fn str_to_graph(s: &str) -> Graph {
    let mut verts_ids = s
        .trim()
        .lines()
        .map(|line| line.split("-"))
        .flatten()
        .map(|s| s.into())
        .collect::<Vec<String>>();
    verts_ids.sort();
    verts_ids.dedup();

    let edges = s
        .trim()
        .lines()
        .map(|line| str_to_edge(line))
        .collect::<Vec<(String, String)>>();

    let mut verts = Vec::new();
    for id in verts_ids {
        verts.push(make_vert(id.to_string(), &edges));
    }

    return Graph { verts: verts };
}

fn str_to_edge(s: &str) -> (String, String) {
    let mut iter = s.split("-");
    let id1 = iter.next().unwrap().into();
    let id2 = iter.next().unwrap().into();
    return (id1, id2);
}

fn make_vert(id: String, edges: &Vec<(String, String)>) -> Vert {
    let big = id.to_uppercase() == id;
    let mut neighbors = Vec::new();
    for (id1, id2) in edges {
        if *id1 == id {
            neighbors.push(id2.to_string());
        }
        if *id2 == id {
            neighbors.push(id1.to_string());
        }
    }
    neighbors.sort();

    return Vert {
        id: id,
        big: big,
        neighbors: neighbors,
    };
}
