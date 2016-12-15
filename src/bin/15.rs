const INPUT: &'static[(usize, usize)] = &[(13, 1),
                                          (19, 10),
                                          (3, 2),
                                          (7, 1),
                                          (5, 3),
                                          (17, 5),
                                          (11, 0),
];

fn main() {
  let answer = (0..).find(|i| {
    INPUT.iter().enumerate().all(|(index, &(size, start))| (index + 1 + start + i) % size == 0)
  }).unwrap();

  println!("{}", answer);
}
