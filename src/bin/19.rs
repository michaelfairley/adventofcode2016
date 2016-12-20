use std::collections::VecDeque;

const CIRCLE_SIZE: i32 = 3001330;
// const CIRCLE_SIZE: i32 = 5;

fn main() {
  let mut circle = VecDeque::new();
  for i in 1..CIRCLE_SIZE+1 {
    circle.push_back((i, 1));
  }

  while circle.len() > 1 {
    let (stealer, before) = circle.pop_front().unwrap();
    let (_, num_stolen) = circle.pop_front().unwrap();

    let after = before + num_stolen;

    circle.push_back((stealer, after));
  }

  println!("{:?}", circle.pop_front().unwrap());
}
