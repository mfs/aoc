use anyhow::Result;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufRead};

const PATH: char = '.';
const FORREST: char = '#';

// slopes are direction of travel
const SLOPE_N: char = '^';
const SLOPE_S: char = 'v';
const SLOPE_E: char = '>';
const SLOPE_W: char = '<';

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<char>>,
    w: usize,
    h: usize,
    start: (usize, usize),
    end: (usize, usize),
}

type Graph = HashMap<((usize, usize), (usize, usize)), usize>;

fn main() -> Result<()> {
    let map: Map = parse()?;

    let vertices = find_vertices(&map);

    let graph = build_graph(&map, &vertices, true);

    println!(
        "Part 1: {}",
        dfs(map.start, &graph, &map, &mut HashSet::new())
    );

    let graph = build_graph(&map, &vertices, false);

    println!(
        "Part 2: {}",
        dfs(map.start, &graph, &map, &mut HashSet::new())
    );

    Ok(())
}

fn build_graph(map: &Map, vertices: &[(usize, usize)], slopes: bool) -> Graph {
    let mut graph: Graph = HashMap::new();

    for (sx, sy) in vertices {
        let mut stack = VecDeque::from([(0, *sx, *sy)]);

        let mut visited = HashSet::from([(*sx, *sy)]);

        while let Some((n, x, y)) = stack.pop_front() {
            if n != 0 && vertices.contains(&(x, y)) {
                graph.insert(((*sx, *sy), (x, y)), n);
                continue;
            }

            for (nx, ny) in neighbours((x, y), &map, slopes) {
                if visited.contains(&(nx, ny)) {
                    continue;
                }

                stack.push_back((n + 1, nx, ny));
                visited.insert((nx, ny));
            }
        }
    }

    graph
}

fn find_vertices(map: &Map) -> Vec<(usize, usize)> {
    let mut vertices = vec![map.start, map.end];

    for y in 0..map.h {
        for x in 0..map.w {
            // always disabling slopes seems to work here.
            // maybe due to vertices always being non slopes?
            if map.grid[y][x] != FORREST && neighbours((x, y), &map, false).len() >= 3 {
                vertices.push((x, y));
            }
        }
    }

    vertices
}

fn dfs(pt: (usize, usize), graph: &Graph, map: &Map, visited: &mut HashSet<(usize, usize)>) -> i32 {
    let mut d = i32::MIN;

    if pt == map.end {
        return 0;
    }

    visited.insert(pt);

    for n in graph.keys().filter(|p| pt.0 == p.0 .0 && pt.1 == p.0 .1) {
        if !visited.contains(&n.1) {
            d = std::cmp::max(d, dfs(n.1, graph, map, visited) + graph[&(pt, n.1)] as i32);
        }
    }

    visited.remove(&pt);

    d
}

fn neighbours(pt: (usize, usize), map: &Map, slopes: bool) -> HashSet<(usize, usize)> {
    let (x, y) = pt;

    if slopes {
        // assumes slopes don't dead end or point off the map
        match map.grid[y][x] {
            SLOPE_N => return HashSet::from([(x, y - 1)]),
            SLOPE_S => return HashSet::from([(x, y + 1)]),
            SLOPE_E => return HashSet::from([(x + 1, y)]),
            SLOPE_W => return HashSet::from([(x - 1, y)]),
            _ => {}
        }
    }

    let mut n = HashSet::new();

    // north
    if y > 0 && map.grid[y - 1][x] != FORREST {
        n.insert((x, y - 1));
    }

    // south
    if y + 1 < map.h && map.grid[y + 1][x] != FORREST {
        n.insert((x, y + 1));
    }

    // east
    if x + 1 < map.w && map.grid[y][x + 1] != FORREST {
        n.insert((x + 1, y));
    }

    // west
    if x > 0 && map.grid[y][x - 1] != FORREST {
        n.insert((x - 1, y));
    }

    n
}

fn parse() -> Result<Map> {
    let mut grid: Vec<_> = vec![];

    let mut start = (0, 0);
    let mut end = (0, 0);

    for line in io::stdin().lock().lines() {
        let line = line?;
        let row: Vec<_> = line.chars().collect();
        grid.push(row);
    }

    let w = grid[0].len();
    let h = grid.len();

    start.0 = grid[0].iter().position(|&c| c == PATH).unwrap();
    end.0 = grid[h - 1].iter().position(|&c| c == PATH).unwrap();
    end.1 = h - 1;

    let map = Map {
        grid,
        w,
        h,
        start,
        end,
    };

    Ok(map)
}
