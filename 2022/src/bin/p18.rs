use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::io::{self, BufRead};

type Pt = (i64, i64, i64);

// face should be (0, 1, 0) etc
const X_POS: i64 = 100;
const X_NEG: i64 = -100;

const Y_POS: i64 = 200;
const Y_NEG: i64 = -200;

const Z_POS: i64 = 300;
const Z_NEG: i64 = -300;

const CUBE_SIDES: usize = 6;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Node {
    loc: Pt,
    face: i64,
}

// should use set for this
fn bfs(cubes: &Vec<Pt>, start: Node) -> usize {
    let mut queue = vec![];
    let mut explored = HashSet::new();

    explored.insert(start);
    queue.push(start);

    while !queue.is_empty() {
        let v = queue.remove(0); // slow

        for w in neighbours(&cubes, v) {
            if !explored.contains(&w) {
                explored.insert(w);
                queue.push(w);
            }
        }
    }

    explored.len()
}

// FIXME well this is a mess
// switching face representation to (0, 0, 1) or (-1, 0, 0)
// will probably help. For now we deal with each face direction
// using an almost duplicate block of logic
fn neighbours(cubes: &Vec<Pt>, node: Node) -> Vec<Node> {
    let mut neighbours = vec![];
    let x = node.loc.0;
    let y = node.loc.1;
    let z = node.loc.2;

    match node.face {
        X_POS => {
            // for each neighbouring in y,z
            'outer: for (yy, zz) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
                // protuding
                if cubes.contains(&(x + 1, y + *yy, z + *zz)) {
                    let new_face = match (*yy, *zz) {
                        (0, 1) => Z_NEG,
                        (0, -1) => Z_POS,
                        (1, 0) => Y_NEG,
                        (-1, 0) => Y_POS,
                        _ => unreachable!(),
                    };
                    neighbours.push(Node {
                        loc: (x + 1, y + *yy, z + *zz),
                        face: new_face,
                    });
                    continue 'outer;
                }

                // flush
                if cubes.contains(&(x, y + *yy, z + *zz)) {
                    neighbours.push(Node {
                        loc: (x, y + *yy, z + *zz),
                        face: X_POS,
                    });
                    continue 'outer;
                }

                // else  use same node and correct face
                let new_face = match (*yy, *zz) {
                    (0, 1) => Z_POS,
                    (0, -1) => Z_NEG,
                    (1, 0) => Y_POS,
                    (-1, 0) => Y_NEG,
                    _ => unreachable!(),
                };
                neighbours.push(Node {
                    loc: node.loc,
                    face: new_face,
                });
            }
        }
        X_NEG => {
            // for each neighbouring in y,z
            'outer: for (yy, zz) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
                // protuding
                if cubes.contains(&(x - 1, y + *yy, z + *zz)) {
                    let new_face = match (*yy, *zz) {
                        (0, 1) => Z_NEG,
                        (0, -1) => Z_POS,
                        (1, 0) => Y_NEG,
                        (-1, 0) => Y_POS,
                        _ => unreachable!(),
                    };
                    neighbours.push(Node {
                        loc: (x - 1, y + *yy, z + *zz),
                        face: new_face,
                    });
                    continue 'outer;
                }

                // flush
                if cubes.contains(&(x, y + *yy, z + *zz)) {
                    neighbours.push(Node {
                        loc: (x, y + *yy, z + *zz),
                        face: X_NEG,
                    });
                    continue 'outer;
                }

                // else  use same node and correct face
                let new_face = match (*yy, *zz) {
                    (0, 1) => Z_POS,
                    (0, -1) => Z_NEG,
                    (1, 0) => Y_POS,
                    (-1, 0) => Y_NEG,
                    _ => unreachable!(),
                };
                neighbours.push(Node {
                    loc: node.loc,
                    face: new_face,
                });
            }
        }
        Y_POS => {
            // for each neighbouring in y,z
            'outer: for (xx, zz) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
                // protuding
                if cubes.contains(&(x + *xx, y + 1, z + *zz)) {
                    let new_face = match (*xx, *zz) {
                        (0, 1) => Z_NEG,
                        (0, -1) => Z_POS,
                        (1, 0) => X_NEG,
                        (-1, 0) => X_POS,
                        _ => unreachable!(),
                    };
                    neighbours.push(Node {
                        loc: (x + *xx, y + 1, z + *zz),
                        face: new_face,
                    });
                    continue 'outer;
                }

                // flush
                if cubes.contains(&(x + *xx, y, z + *zz)) {
                    neighbours.push(Node {
                        loc: (x + *xx, y, z + *zz),
                        face: Y_POS,
                    });
                    continue 'outer;
                }

                // else  use same node and correct face
                let new_face = match (*xx, *zz) {
                    (0, 1) => Z_POS,
                    (0, -1) => Z_NEG,
                    (1, 0) => X_POS,
                    (-1, 0) => X_NEG,
                    _ => unreachable!(),
                };
                neighbours.push(Node {
                    loc: node.loc,
                    face: new_face,
                });
            }
        }
        Y_NEG => {
            // for each neighbouring in y,z
            'outer: for (xx, zz) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
                // protuding
                if cubes.contains(&(x + *xx, y - 1, z + *zz)) {
                    let new_face = match (*xx, *zz) {
                        (0, 1) => Z_NEG,
                        (0, -1) => Z_POS,
                        (1, 0) => X_NEG,
                        (-1, 0) => X_POS,
                        _ => unreachable!(),
                    };
                    neighbours.push(Node {
                        loc: (x + *xx, y - 1, z + *zz),
                        face: new_face,
                    });
                    continue 'outer;
                }

                // flush
                if cubes.contains(&(x + *xx, y, z + *zz)) {
                    neighbours.push(Node {
                        loc: (x + *xx, y, z + *zz),
                        face: Y_NEG,
                    });
                    continue 'outer;
                }

                // else  use same node and correct face
                let new_face = match (*xx, *zz) {
                    (0, 1) => Z_POS,
                    (0, -1) => Z_NEG,
                    (1, 0) => X_POS,
                    (-1, 0) => X_NEG,
                    _ => unreachable!(),
                };
                neighbours.push(Node {
                    loc: node.loc,
                    face: new_face,
                });
            }
        }
        Z_POS => {
            // for each neighbouring in x,y
            'outer: for (xx, yy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
                // protuding
                if cubes.contains(&(x + *xx, y + *yy, z + 1)) {
                    let new_face = match (*xx, *yy) {
                        (0, 1) => Y_NEG,
                        (0, -1) => Y_POS,
                        (1, 0) => X_NEG,
                        (-1, 0) => X_POS,
                        _ => unreachable!(),
                    };
                    neighbours.push(Node {
                        loc: (x + *xx, y + *yy, z + 1),
                        face: new_face,
                    });
                    continue 'outer;
                }

                // flush
                if cubes.contains(&(x + *xx, y + *yy, z)) {
                    neighbours.push(Node {
                        loc: (x + *xx, y + *yy, z),
                        face: Z_POS,
                    });
                    continue 'outer;
                }

                // else  use same node and correct face
                let new_face = match (*xx, *yy) {
                    (0, 1) => Y_POS,
                    (0, -1) => Y_NEG,
                    (1, 0) => X_POS,
                    (-1, 0) => X_NEG,
                    _ => unreachable!(),
                };
                neighbours.push(Node {
                    loc: node.loc,
                    face: new_face,
                });
            }
        }
        Z_NEG => {
            // for each neighbouring in x,y
            'outer: for (xx, yy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
                // protuding
                if cubes.contains(&(x + *xx, y + *yy, z - 1)) {
                    let new_face = match (*xx, *yy) {
                        (0, 1) => Y_NEG,
                        (0, -1) => Y_POS,
                        (1, 0) => X_NEG,
                        (-1, 0) => X_POS,
                        _ => unreachable!(),
                    };
                    neighbours.push(Node {
                        loc: (x + *xx, y + *yy, z - 1),
                        face: new_face,
                    });
                    continue 'outer;
                }

                // flush
                if cubes.contains(&(x + *xx, y + *yy, z)) {
                    neighbours.push(Node {
                        loc: (x + *xx, y + *yy, z),
                        face: Z_NEG,
                    });
                    continue 'outer;
                }

                // else  use same node and correct face
                let new_face = match (*xx, *yy) {
                    (0, 1) => Y_POS,
                    (0, -1) => Y_NEG,
                    (1, 0) => X_POS,
                    (-1, 0) => X_NEG,
                    _ => unreachable!(),
                };
                neighbours.push(Node {
                    loc: node.loc,
                    face: new_face,
                });
            }
        }
        _ => unreachable!(),
    }

    neighbours
}

fn main() -> Result<()> {
    let mut cubes: Vec<Pt> = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        let tokens: Vec<_> = line.split(",").filter_map(|s| s.parse().ok()).collect();

        cubes.push((tokens[0], tokens[1], tokens[2]));
    }

    let n = cubes.len();

    let mut common_sides = 0;

    for a in &cubes {
        for b in &cubes {
            if a != b && is_connected(*a, *b) {
                common_sides += 1;
            }
        }
    }

    let part1 = CUBE_SIDES * n - common_sides;

    println!("Part 1: {}", part1);

    // find a start node
    let max_x = cubes.iter().map(|c| c.0).max().ok_or(anyhow!("no cubes"))?;

    let max_x_cubes: Vec<_> = cubes.iter().filter(|&c| c.0 == max_x).collect();

    let start = Node {
        loc: *max_x_cubes[0],
        face: X_POS,
    };

    println!("Part 2: {}", bfs(&cubes, start));

    Ok(())
}

fn is_connected(a: Pt, b: Pt) -> bool {
    if a.0 == b.0 && a.1 == b.1 {
        return (a.2 - b.2).abs() == 1;
    }

    if a.0 == b.0 && a.2 == b.2 {
        return (a.1 - b.1).abs() == 1;
    }

    if a.1 == b.1 && a.2 == b.2 {
        return (a.0 - b.0).abs() == 1;
    }

    false
}
