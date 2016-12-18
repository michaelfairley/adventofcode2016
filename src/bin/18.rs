// const INPUT: &'static str = ".^^.^.^^^^";
const INPUT: &'static str = ".^^^^^.^^.^^^.^...^..^^.^.^..^^^^^^^^^^..^...^^.^..^^^^..^^^^...^.^.^^^^^^^^....^..^^^^^^.^^^.^^^.^^";

fn next(prev: &Vec<bool>) -> Vec<bool> {
  let mut result = vec![];

  result.push(prev[1]);
  result.extend(prev.windows(3).map(|w| w[0] ^ w[2]));
  result.push(prev[prev.len() - 2]);

  result
}

fn main() {
  let start = INPUT.chars().map(|c| c == '^' ).collect::<Vec<_>>();

  let result = (0..400_000).scan(start, |v, _| {
    let n = next(v);
    let res = ::std::mem::replace(v, n);
    Some(res)
  }).flat_map(|v| v.into_iter()).filter(|b| !b).count();

  println!("{}", result);
}
