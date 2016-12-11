use std::collections::{VecDeque,HashSet};

#[derive(Hash,PartialEq,Eq,Clone,Copy)]
enum Element {
  // Hydrogen, // Sample
  // Lithium, // Sample
  Promethium,
  Cobalt,
  Curium,
  Ruthenium,
  Plutonium,
  Elerium, // Part 2
  Dilithium, // Part 2
}

#[derive(Hash,PartialEq,Eq,Clone,Copy)]
enum Kind {
  Microchip,
  Generator,
}

use Element::*;
use Kind::*;

#[derive(Hash,PartialEq,Eq,Clone)]
struct Thing {
  floor: i32,
  element: Element,
  kind: Kind,
}

#[derive(Hash,PartialEq,Eq,Clone)]
struct State {
  elevator: i32,
  things: Vec<Thing>,
}

impl State {
  fn valid(&self) -> bool {
    self.things.iter()
      .filter(|t| t.kind == Microchip)
      .filter(|t| !self.things.iter().any(|t2| t2.kind == Generator && t2.floor == t.floor && t2.element == t.element))
      .all(|t| !self.things.iter().any(|t2| t2.kind == Generator && t2.floor == t.floor))
  }

  fn won(&self) -> bool {
    self.things.iter().all(|t| t.floor == 4)
  }

  fn neighbors(&self) -> Vec<State> {
    let mut valid_floors = vec![];
    if self.elevator > 1 { valid_floors.push(self.elevator-1) }
    if self.elevator < 4 { valid_floors.push(self.elevator+1) }
    let valid_floors = valid_floors;

    let mut result = vec![];

    for (i, _) in self.things.iter().enumerate().filter(|&(_,t)| t.floor == self.elevator) {
      for (j, _) in self.things.iter().enumerate().filter(|&(_,t)| t.floor == self.elevator) {
        for &new_floor in &valid_floors {
          let mut new_state = self.clone();
          new_state.elevator = new_floor;
          new_state.things[i].floor = new_floor;
          new_state.things[j].floor = new_floor;
          result.push(new_state);
        }
      }
    }

    result
  }
}

fn main() {
  let initial = State {
    elevator: 1,
    things: vec![
      // Thing{ floor: 1, element: Hydrogen, kind: Microchip }, // Sample
      // Thing{ floor: 1, element: Lithium,  kind: Microchip }, // Sample
      // Thing{ floor: 2, element: Hydrogen, kind: Generator }, // Sample
      // Thing{ floor: 3, element: Lithium,  kind: Generator }, // Sample
      Thing{ floor: 1, element: Promethium, kind: Microchip },
      Thing{ floor: 1, element: Promethium, kind: Generator },
      Thing{ floor: 1, element: Elerium, kind: Microchip }, // Part 2
      Thing{ floor: 1, element: Elerium, kind: Generator }, // Part 2
      Thing{ floor: 1, element: Dilithium, kind: Microchip }, // Part 2
      Thing{ floor: 1, element: Dilithium, kind: Generator }, // Part 2
      Thing{ floor: 2, element: Cobalt, kind: Generator },
      Thing{ floor: 2, element: Curium, kind: Generator },
      Thing{ floor: 2, element: Ruthenium, kind: Generator },
      Thing{ floor: 2, element: Plutonium, kind: Generator },
      Thing{ floor: 3, element: Cobalt, kind: Microchip },
      Thing{ floor: 3, element: Curium, kind: Microchip },
      Thing{ floor: 3, element: Ruthenium, kind: Microchip },
      Thing{ floor: 3, element: Plutonium, kind: Microchip },
    ],
  };

  let mut pending = VecDeque::new();
  pending.push_back((initial.clone(), 0));

  let mut known = HashSet::new();
  known.insert(initial);

  while let Some((candidate, moves)) = pending.pop_front() {
    if candidate.won() {
      println!("{}", moves);
      return;
    }

    for next in candidate.neighbors() {
        if next.valid() && !known.contains(&next) {
          known.insert(next.clone());
          pending.push_back((next, moves+1));
        }
      }
  }

}
