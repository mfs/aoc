use std::io::{self, BufRead};
use std::cmp::{min, max};
use std::collections::HashMap;
use anyhow::Result;

//       -- no stop --
//       |   |   |   |
//       v   v   v   v
//   0 1 2 3 4 5 6 7 8 9 10
//     | r | r | r | r |
//     | 0 | 1 | 2 | 3 |

const NUMBER_OF_ROOMS: usize = 4;

const TYPE: [char; 4] = ['A', 'B', 'C', 'D'];

const ROOM_HALLWAY: [usize; 4] = [2, 4, 6, 8];

const HALLWAY_STOPS: [usize; 11 - 4] = [0, 1, 3, 5, 7, 9, 10];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GameState {
    hallway: [char; 11],
    rooms: [Vec<char>; NUMBER_OF_ROOMS],
    cost: u64,
    room_size: usize,
}

impl GameState {
    fn new(r1: &[char], r2: &[char], r3: &[char], r4: &[char]) -> Self {
        GameState {
            hallway: ['.'; 11],
            rooms: [r1.to_vec(), r2.to_vec(), r3.to_vec(), r4.to_vec()],
            cost: 0,
            room_size: r1.len(),
        }
    }

    fn room_open(&self, r: usize) -> bool {
        if self.rooms[r].is_empty() {
            return true;
        }

        if self.rooms[r].iter().all(|&x| x == TYPE[r]) && self.rooms[r].len() < self.room_size {
            return true;
        }

        false
    }

    // check if hallway passable from start to end inclusive
    fn hallway_open(&self, mut start: usize, end: usize, include_start: bool) -> bool {
        if !include_start {
            if start < end {
                start += 1;
            } else {
                start -= 1;
            }
        }
        let (x0, x1) = (min(start, end), max(start, end));
        assert!(x0 <= x1);
        for i in x0..=x1 {
            if self.hallway[i] != '.' {
                return false;
            }
        }
        true
    }

    fn room_complete(&self, room: usize) -> bool {
        self.rooms[room].len() == self.room_size && self.rooms[room].iter().all(|&c| c == TYPE[room])
    }

    fn rooms_complete(&self) -> bool {
        for i in 0..4 {
            if !self.room_complete(i) {
                return false;
            }
        }
        true
    }
}

fn cost(amphipod: char) -> u64 {
    match amphipod {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => unreachable!(),
    }
}

fn dst(amphipod: char) -> usize {
    match amphipod {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        _ => unreachable!(),
    }
}

fn main() -> Result<()> {
    const ROOM_COLUMNS: [usize; 4] = [3, 5, 7, 9];

    let mut rooms = [vec![], vec![], vec![], vec![]];

    for line in io::stdin().lock().lines() {
        let cells: Vec<_> = line?.chars().collect();
        if TYPE.contains(&cells[3]) {
            for i in 0..4 {
                rooms[i].insert(0, cells[ROOM_COLUMNS[i]]);
            }
        }
    }

    let state = GameState::new(&rooms[0], &rooms[1], &rooms[2], &rooms[3]);

    let mut cache:HashMap<GameState, u64> = HashMap::new();

    println!("Part 1: {}", solve(&mut cache, &state));

    for (i, cells) in [['D', 'D'], ['B', 'C'], ['A', 'B'], ['C', 'A']].iter().enumerate() {
        rooms[i].insert(1, cells[1]);
        rooms[i].insert(1, cells[0]);
    }

    let state = GameState::new(&rooms[0], &rooms[1], &rooms[2], &rooms[3]);

    let mut cache:HashMap<GameState, u64> = HashMap::new();

    println!("Part 2: {}", solve(&mut cache, &state));

    Ok(())
}

fn solve(cache: &mut HashMap<GameState, u64>, state: &GameState) -> u64 {
    if state.rooms_complete() {
        return state.cost;
    }

    if let Some(x) = cache.get(&state) {
        return *x;
    }

    let mut min_cost = u64::MAX;

    // check hallway to dest room moves
    for (src_idx, c) in state.hallway.iter().enumerate().filter(|(_, &c)| c != '.') {
        let dst_room_idx = dst(*c);
        let dst_idx = ROOM_HALLWAY[dst(*c)];

        if state.hallway_open(src_idx, dst_idx, false) && state.room_open(dst_room_idx) {
            let mut next_state = state.clone();
            let mut moves = delta(src_idx, dst_idx) - 1; // don't count start square

            // moves for room
            moves += state.room_size - state.rooms[dst_room_idx].len();

            // update cost
            next_state.cost += moves as u64 * cost(*c);

            // update room
            next_state.rooms[dst_room_idx].push(*c);

            // clear hallway
            next_state.hallway[src_idx] = '.';

            // recurse
            min_cost = min(min_cost, solve(cache, &next_state));
        }
    }

    // check room to hallway moves
    for (src_room_idx, room) in state.rooms.iter().enumerate() {
        // look at rooms that have non home pods anywhere
        if room.iter().any(|&x| x != TYPE[src_room_idx] ) {
            // for each hallway stop
            for hallway_dst_idx in HALLWAY_STOPS {
                // not vacant
                if state.hallway[hallway_dst_idx] != '.' {
                    continue;
                }
                // can we move there?
                if !state.hallway_open(ROOM_HALLWAY[src_room_idx], hallway_dst_idx, true) {
                    continue;
                }

                let mut moves = delta(ROOM_HALLWAY[src_room_idx], hallway_dst_idx);
                moves += state.room_size - room.len();

                let mut next_state = state.clone();

                // update new state
                let pod = next_state.rooms[src_room_idx].pop().unwrap();
                next_state.hallway[hallway_dst_idx] = pod;

                next_state.cost += moves as u64 * cost(pod);

                min_cost = min(min_cost, solve(cache, &next_state));
            }
        }
    }

    cache.insert(state.clone(), min_cost);
    min_cost
}

fn delta(x0: usize, x1: usize) -> usize {
    let (x0, x1) = (min(x0, x1), max(x0, x1));

    x1 - x0 + 1
}

