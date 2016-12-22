extern crate regex;
use regex::Regex;

// const INPUT: &'static str = "swap position 4 with position 0
// swap letter d with letter b
// reverse positions 0 through 4
// rotate left 1 step
// move position 1 to position 4
// move position 3 to position 0
// rotate based on position of letter b
// rotate based on position of letter d";
const INPUT: &'static str = "swap position 5 with position 6
reverse positions 1 through 6
rotate right 7 steps
rotate based on position of letter c
rotate right 7 steps
reverse positions 0 through 4
swap letter f with letter h
reverse positions 1 through 2
move position 1 to position 0
rotate based on position of letter f
move position 6 to position 3
reverse positions 3 through 6
rotate based on position of letter c
rotate based on position of letter b
move position 2 to position 4
swap letter b with letter d
move position 1 to position 6
move position 7 to position 1
swap letter f with letter c
move position 2 to position 3
swap position 1 with position 7
reverse positions 3 through 5
swap position 1 with position 4
move position 4 to position 7
rotate right 4 steps
reverse positions 3 through 6
move position 0 to position 6
swap position 3 with position 5
swap letter e with letter h
rotate based on position of letter c
swap position 4 with position 7
reverse positions 0 through 5
rotate right 5 steps
rotate left 0 steps
rotate based on position of letter f
swap letter e with letter b
rotate right 2 steps
rotate based on position of letter c
swap letter a with letter e
rotate left 4 steps
rotate left 0 steps
move position 6 to position 7
rotate right 2 steps
rotate left 6 steps
rotate based on position of letter d
swap letter a with letter b
move position 5 to position 4
reverse positions 0 through 7
rotate left 3 steps
rotate based on position of letter e
rotate based on position of letter h
swap position 4 with position 6
reverse positions 4 through 5
reverse positions 5 through 7
rotate left 3 steps
move position 7 to position 2
move position 3 to position 4
swap letter b with letter d
reverse positions 3 through 4
swap letter e with letter a
rotate left 4 steps
swap position 3 with position 4
swap position 7 with position 5
rotate right 1 step
rotate based on position of letter g
reverse positions 0 through 3
swap letter g with letter b
rotate based on position of letter b
swap letter a with letter c
swap position 0 with position 2
reverse positions 1 through 3
rotate left 7 steps
swap letter f with letter a
move position 5 to position 0
reverse positions 1 through 5
rotate based on position of letter d
rotate based on position of letter c
rotate left 2 steps
swap letter b with letter a
swap letter f with letter c
swap letter h with letter f
rotate based on position of letter b
rotate left 3 steps
swap letter b with letter h
reverse positions 1 through 7
rotate based on position of letter h
swap position 1 with position 5
rotate left 1 step
rotate based on position of letter h
reverse positions 0 through 1
swap position 5 with position 7
reverse positions 0 through 2
reverse positions 1 through 3
move position 1 to position 4
reverse positions 1 through 3
rotate left 1 step
swap position 4 with position 1
move position 1 to position 3
rotate right 2 steps
move position 0 to position 5
";

fn main() {
  let swapp = Regex::new(r"swap position (?P<x>\d+) with position (?P<y>\d+)").unwrap();
  let swapl = Regex::new(r"swap letter (?P<x>\w) with letter (?P<y>\w)").unwrap();
  let rotate = Regex::new(r"rotate (?P<dir>\w+) (?P<n>\d+) steps?").unwrap();
  let rotatep = Regex::new(r"rotate based on position of letter (?P<x>\w)").unwrap();
  let reverse = Regex::new(r"reverse positions (?P<x>\d+) through (?P<y>\d+)").unwrap();
  let mov = Regex::new(r"move position (?P<x>\d+) to position (?P<y>\d+)").unwrap();

  let mut password = vec![];
  // password.extend_from_slice(b"abcde");
  password.extend_from_slice(b"abcdefgh");

  for line in INPUT.lines() {
    if let Some(captures) = swapp.captures(line) {
      let x = captures["x"].parse().unwrap();
      let y = captures["y"].parse().unwrap();
      password.swap(x, y);
    } else if let Some(captures) = swapl.captures(line) {
      let x = captures["x"].bytes().next().unwrap();
      let y = captures["y"].bytes().next().unwrap();
      let xi = password.iter().position(|&c| c == x ).unwrap();
      let yi = password.iter().position(|&c| c == y ).unwrap();
      password.swap(xi, yi);
    } else if let Some(captures) = rotate.captures(line) {
      let left = &captures["dir"] == "left";
      let mut amount = captures["n"].parse::<i32>().unwrap();
      if !left { amount *= -1 }
      while amount < 0 { amount += password.len() as i32 }
      for _ in 0..amount {
        let z = password.remove(0);
        password.push(z);
      }
    } else if let Some(captures) = rotatep.captures(line) {
      let x = captures["x"].bytes().next().unwrap();
      let xi = password.iter().position(|&c| c == x ).unwrap();
      let amount = 1 + xi + if xi >= 4 { 1 } else { 0 };
      for _ in 0..amount {
        let z = password.pop().unwrap();
        password.insert(0, z);
      }
    } else if let Some(captures) = reverse.captures(line) {
      let x = captures["x"].parse::<usize>().unwrap();
      let y = captures["y"].parse::<usize>().unwrap();
      password[x..y+1].reverse();
    } else if let Some(captures) = mov.captures(line) {
      let x = captures["x"].parse::<usize>().unwrap();
      let y = captures["y"].parse::<usize>().unwrap();

      let z = password.remove(x);
      password.insert(y, z);
    } else {
      println!("{}", line);
      unimplemented!();
    }
  }

  println!("{}", String::from_utf8(password).unwrap());
}
