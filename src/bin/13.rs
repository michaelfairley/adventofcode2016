use std::collections::{HashSet,VecDeque};

// const INPUT: u32 = 10;
// const GOAL: (u32, u32) = (7, 4);
const INPUT: u32 = 1364;
const GOAL: (u32, u32) = (31, 39);

fn open(coords: (u32, u32)) -> bool {
  let (x, y) = coords;
  (x*x + 3*x + 2*x*y + y + y*y + INPUT).count_ones() % 2 == 0
}

fn main() {

  let mut pending = VecDeque::new();
  let mut known = HashSet::new();

  {
    let initial = (1, 1);
    pending.push_back((initial, 0));
    known.insert(initial);
  }

  while let Some((candidate, moves)) = pending.pop_front() {
    if candidate == GOAL {
      println!("{}", moves);
      return;
    }

    let mut neighbors = vec![];
                         neighbors.push((candidate.0 + 1, candidate.1    ));
                         neighbors.push((candidate.0    , candidate.1 + 1));
    if candidate.0 > 0 { neighbors.push((candidate.0 - 1, candidate.1    )); }
    if candidate.1 > 0 { neighbors.push((candidate.0    , candidate.1 - 1)); }

    for neighbor in neighbors {
      if open(neighbor) && !known.contains(&neighbor) {
        known.insert(neighbor);
        pending.push_back((neighbor, moves+1));
      }
    }
  }
}
