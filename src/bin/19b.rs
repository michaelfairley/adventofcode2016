use std::collections::LinkedList;

const CIRCLE_SIZE: usize = 3001330;
const CIRCLE_SIZE: usize = 5;

fn main() {
  let mut circle = LinkedList::new();
  for i in 1..CIRCLE_SIZE+1 {
    circle.push_back((i, 1));
  }
  let mut back = circle.split_off(CIRCLE_SIZE/2-1);

  while circle.len() + back.len() > 1 {

    if (circle.len() + back.len()) % 2 == 0 {
      circle.push_back(back.pop_front().unwrap());
    }

    let (stealer, before) = circle.pop_front().unwrap();
    let (_, num_stolen) = back.pop_front().unwrap();


    let after = before + num_stolen;

    back.push_back((stealer, after));
  }

  println!("{:?}", back.pop_front().unwrap());
}
