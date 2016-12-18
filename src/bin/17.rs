extern crate crypto;

use crypto::digest::Digest;
use std::collections::VecDeque;

const INPUT: &'static str = "dmypynyp";

fn location(s: &str) -> (u8, u8) {
  let mut x = 1;
  let mut y = 1;

  for dir in s.bytes() {
    match dir {
      b'D' => { y += 1 },
      b'U' => { y -= 1 },
      b'L' => { x -= 1 },
      b'R' => { x += 1 },
      _ => unimplemented!(),
    }
  }

  (x, y)
}

fn dirs(s: &str) -> Vec<char> {
  let mut md5 = ::crypto::md5::Md5::new();

  let mut result = vec![];

  md5.reset();
  md5.input_str(INPUT);
  md5.input_str(&s.to_string());

  let hash = md5.result_str();
  let mut bytes = hash.bytes();

  if bytes.next().unwrap() >= b'b' { result.push('U') }
  if bytes.next().unwrap() >= b'b' { result.push('D') }
  if bytes.next().unwrap() >= b'b' { result.push('L') }
  if bytes.next().unwrap() >= b'b' { result.push('R') }

  result
}

fn main() {
  let mut pending = VecDeque::new();
  pending.push_back(String::new());

  while let Some(candidate) = pending.pop_front() {
    let loc = location(&candidate);
    if loc == (4, 4) {
      println!("{}", candidate);
      return;
    }

    for dir in dirs(&candidate) {
      if dir == 'U' && loc.1 > 1 {
        let mut s = String::new();
        s.push_str(&candidate);
        s.push(dir);
        pending.push_back(s);
      } else if dir == 'D' && loc.1 < 4 {
        let mut s = String::new();
        s.push_str(&candidate);
        s.push(dir);
        pending.push_back(s);
      } else if dir == 'L' && loc.0 > 1 {
        let mut s = String::new();
        s.push_str(&candidate);
        s.push(dir);
        pending.push_back(s);
      } else if dir == 'R' && loc.0 < 4 {
        let mut s = String::new();
        s.push_str(&candidate);
        s.push(dir);
        pending.push_back(s);
      }
    }
  }
}
