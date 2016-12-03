const INPUT: &'static str = "L5, R1, R3, L4, R3, R1, L3, L2, R3, L5, L1, L2, R5, L1, R5, R1, L4, R1, R3, L4, L1, R2, R5, R3, R1, R1, L1, R1, L1, L2, L1, R2, L5, L188, L4, R1, R4, L3, R47, R1, L1, R77, R5, L2, R1, L2, R4, L5, L1, R3, R187, L4, L3, L3, R2, L3, L5, L4, L4, R1, R5, L4, L3, L3, L3, L2, L5, R1, L2, R5, L3, L4, R4, L5, R3, R4, L2, L1, L4, R1, L3, R1, R3, L2, R1, R4, R5, L3, R5, R3, L3, R4, L2, L5, L1, L1, R3, R1, L4, R3, R3, L2, R5, R4, R1, R3, L4, R3, R3, L2, L4, L5, R1, L4, L5, R4, L2, L1, L3, L3, L5, R3, L4, L3, R5, R4, R2, L4, R2, R3, L3, R4, L1, L3, R2, R1, R5, L4, L5, L5, R4, L5, L2, L4, R4, R4, R1, L3, L2, L4, R3";

fn main() {
  let mut direction = 0;
  let mut x: i32 = 0;
  let mut y: i32 = 0;

  for command in INPUT.split(", ") {
    let (dir, len) = command.split_at(1);
    let len = len.parse::<i32>().unwrap();

    direction += if dir == "L" { -1 } else { 1 };
    direction = (direction + 4) % 4;

    match direction {
      0 => x += len,
      1 => y += len,
      2 => x -= len,
      3 => y -= len,
      _ => unreachable!(),
    }
  }

  println!("{}", x.abs() + y.abs());
}
