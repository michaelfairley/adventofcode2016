extern crate crypto;

use crypto::digest::Digest;

const INPUT: &'static[u8] = b"uqwqemis";
// const INPUT: &'static[u8] = b"abc";

fn main() {
  let mut md5 = ::crypto::md5::Md5::new();

  let password = String::from_utf8((0..).filter_map(
    |i| {
      md5.reset();
      md5.input(INPUT);
      md5.input_str(&i.to_string());

      let hash = md5.result_str();
      if hash.starts_with("00000") && hash.as_bytes()[5] >= b'0' && hash.as_bytes()[5] <= b'7' {
        Some((hash.as_bytes()[5] - b'0', hash.as_bytes()[6]))
      } else { None }
    }).scan([0, 0, 0, 0, 0, 0, 0, 0], |s, (i, v)| {
            if s[i as usize] == 0 {
              s[i as usize] = v;
            }
            Some(*s)
    }).filter(|s| s.iter().all(|v| *v > 0))
                                   .next().unwrap().to_vec()).unwrap();

  println!("{}", password);
}
