extern crate crypto;

use crypto::digest::Digest;
use std::collections::HashMap;

// const INPUT: &'static[u8] = b"abc";
const INPUT: &'static[u8] = b"qzyelonm";

fn main() {
  let mut md5 = ::crypto::md5::Md5::new();

  let mut hash = |n: i32| -> String {
    md5.reset();
    md5.input(INPUT);
    md5.input_str(&n.to_string());

    (0..2016).fold(md5.result_str(), |acc, _| {
      md5.reset();
      md5.input_str(&acc);
      md5.result_str()
    })
  };

  let mut memo: HashMap<i32, String> = HashMap::new();
  let mut mhash = |n: i32| -> String {
    memo.entry(n).or_insert_with(|| hash(n)).clone()
  };

  let result = (0..).filter(|i| {
    let ohash = mhash(*i);
    let bytes = ohash.bytes().collect::<Vec<_>>();

    bytes.windows(3).filter(|window| window[0] == window[1] && window[0] == window[2]).next().map(|window| {
      (i+1..i+1000).any(|j: i32| {
        let ihash = mhash(j);
        let bytes = ihash.bytes().collect::<Vec<_>>();

        bytes.windows(5).any(|iwindow| {
          window[0] == iwindow[0] && window[0] == iwindow[1] && window[0] == iwindow[2] && window[0] == iwindow[3] && window[0] == iwindow[4]
        })
      })
    }).unwrap_or(false)
  }).nth(63).unwrap();

  println!("{}", result);
}
