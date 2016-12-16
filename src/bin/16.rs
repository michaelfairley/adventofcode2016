// const LENGTH: usize = 20;
// const INPUT: &'static [u8] = b"10000";
// const LENGTH: usize = 272;
const LENGTH: usize = 35651584;
const INPUT: &'static [u8] = b"11100010111110100";

fn dragon(data: Vec<u8>) -> Vec<u8> {
  let mut result = Vec::with_capacity(data.len() * 2 + 1);

  result.extend_from_slice(&data);
  result.push(b'0');
  result.extend(data.iter().rev().map(|&b| if b == b'0' { b'1' } else { b'0' }));

  result
}

fn checksum(data: &Vec<u8>) -> Vec<u8> {
  data.chunks(2)
    .map(|w| if w[0] == w[1] { b'1' } else { b'0' })
    .collect::<Vec<_>>()
}

fn main() {
  let mut data = Vec::new();
  data.extend_from_slice(INPUT);

  while data.len() < LENGTH {
    data = dragon(data);
  }

  data.truncate(LENGTH);

  let mut sum = checksum(&data);

  while sum.len() % 2 == 0 {
    sum = checksum(&sum)
  }

  println!("{}", String::from_utf8(sum).unwrap());
}
