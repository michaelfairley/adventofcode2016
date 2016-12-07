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
      if hash.starts_with("00000") {
        Some(hash.as_bytes()[5])
      } else { None }
    }).take(8)
                                   .collect::<Vec<u8>>()).unwrap();

  println!("{}", password);
}
