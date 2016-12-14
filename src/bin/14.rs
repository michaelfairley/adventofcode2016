extern crate crypto;

use crypto::digest::Digest;

// const INPUT: &'static[u8] = b"abc";
const INPUT: &'static[u8] = b"qzyelonm";

fn main() {
  let mut md5 = ::crypto::md5::Md5::new();

  let result = (0..).filter(|i| {
    md5.reset();
    md5.input(INPUT);
    md5.input_str(&i.to_string());

    let hash = md5.result_str();
    let bytes = hash.bytes().collect::<Vec<_>>();

    bytes.windows(3).filter(|window| window[0] == window[1] && window[0] == window[2]).next().map(|window| {
      (i+1..i+1000).any(|j: i32| {
        md5.reset();
        md5.input(INPUT);
        md5.input_str(&j.to_string());

        let hash = md5.result_str();
        let bytes = hash.bytes().collect::<Vec<_>>();

        bytes.windows(5).any(|iwindow| {
          window[0] == iwindow[0] && window[0] == iwindow[1] && window[0] == iwindow[2] && window[0] == iwindow[3] && window[0] == iwindow[4]
        })
      })
    }).unwrap_or(false)
  }).nth(63).unwrap();

  println!("{}", result);
}
