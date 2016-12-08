extern crate regex;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

const INPUT: &'static str = "rect 1x1
rotate row y=0 by 6
rect 1x1
rotate row y=0 by 3
rect 1x1
rotate row y=0 by 5
rect 1x1
rotate row y=0 by 4
rect 2x1
rotate row y=0 by 5
rect 2x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 5
rect 4x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 3
rect 1x1
rotate row y=0 by 3
rect 1x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 6
rect 4x1
rotate row y=0 by 4
rotate column x=0 by 1
rect 3x1
rotate row y=0 by 6
rotate column x=0 by 1
rect 4x1
rotate column x=10 by 1
rotate row y=2 by 16
rotate row y=0 by 8
rotate column x=5 by 1
rotate column x=0 by 1
rect 7x1
rotate column x=37 by 1
rotate column x=21 by 2
rotate column x=15 by 1
rotate column x=11 by 2
rotate row y=2 by 39
rotate row y=0 by 36
rotate column x=33 by 2
rotate column x=32 by 1
rotate column x=28 by 2
rotate column x=27 by 1
rotate column x=25 by 1
rotate column x=22 by 1
rotate column x=21 by 2
rotate column x=20 by 3
rotate column x=18 by 1
rotate column x=15 by 2
rotate column x=12 by 1
rotate column x=10 by 1
rotate column x=6 by 2
rotate column x=5 by 1
rotate column x=2 by 1
rotate column x=0 by 1
rect 35x1
rotate column x=45 by 1
rotate row y=1 by 28
rotate column x=38 by 2
rotate column x=33 by 1
rotate column x=28 by 1
rotate column x=23 by 1
rotate column x=18 by 1
rotate column x=13 by 2
rotate column x=8 by 1
rotate column x=3 by 1
rotate row y=3 by 2
rotate row y=2 by 2
rotate row y=1 by 5
rotate row y=0 by 1
rect 1x5
rotate column x=43 by 1
rotate column x=31 by 1
rotate row y=4 by 35
rotate row y=3 by 20
rotate row y=1 by 27
rotate row y=0 by 20
rotate column x=17 by 1
rotate column x=15 by 1
rotate column x=12 by 1
rotate column x=11 by 2
rotate column x=10 by 1
rotate column x=8 by 1
rotate column x=7 by 1
rotate column x=5 by 1
rotate column x=3 by 2
rotate column x=2 by 1
rotate column x=0 by 1
rect 19x1
rotate column x=20 by 3
rotate column x=14 by 1
rotate column x=9 by 1
rotate row y=4 by 15
rotate row y=3 by 13
rotate row y=2 by 15
rotate row y=1 by 18
rotate row y=0 by 15
rotate column x=13 by 1
rotate column x=12 by 1
rotate column x=11 by 3
rotate column x=10 by 1
rotate column x=8 by 1
rotate column x=7 by 1
rotate column x=6 by 1
rotate column x=5 by 1
rotate column x=3 by 2
rotate column x=2 by 1
rotate column x=1 by 1
rotate column x=0 by 1
rect 14x1
rotate row y=3 by 47
rotate column x=19 by 3
rotate column x=9 by 3
rotate column x=4 by 3
rotate row y=5 by 5
rotate row y=4 by 5
rotate row y=3 by 8
rotate row y=1 by 5
rotate column x=3 by 2
rotate column x=2 by 3
rotate column x=1 by 2
rotate column x=0 by 2
rect 4x2
rotate column x=35 by 5
rotate column x=20 by 3
rotate column x=10 by 5
rotate column x=3 by 2
rotate row y=5 by 20
rotate row y=3 by 30
rotate row y=2 by 45
rotate row y=1 by 30
rotate column x=48 by 5
rotate column x=47 by 5
rotate column x=46 by 3
rotate column x=45 by 4
rotate column x=43 by 5
rotate column x=42 by 5
rotate column x=41 by 5
rotate column x=38 by 1
rotate column x=37 by 5
rotate column x=36 by 5
rotate column x=35 by 1
rotate column x=33 by 1
rotate column x=32 by 5
rotate column x=31 by 5
rotate column x=28 by 5
rotate column x=27 by 5
rotate column x=26 by 5
rotate column x=17 by 5
rotate column x=16 by 5
rotate column x=15 by 4
rotate column x=13 by 1
rotate column x=12 by 5
rotate column x=11 by 5
rotate column x=10 by 1
rotate column x=8 by 1
rotate column x=2 by 5
rotate column x=1 by 5
";

// const INPUT: &'static str = "rect 3x2
// rotate column x=1 by 1
// rotate row y=0 by 4
// rotate column x=1 by 1";

fn viz(grid: [[bool; WIDTH]; HEIGHT]) {
  for row in grid.iter() {
    let s = row.iter().map(|&i| if i { "#" } else { "." }).collect::<String>();
    println!("{}", s);
  }
  println!("");
}

fn main() {
  use regex::Regex;
  let rect = Regex::new(r"^rect (?P<w>.+)x(?P<h>.+)$").unwrap();
  let rcol = Regex::new(r"^rotate column x=(?P<x>.+) by (?P<n>.+)$").unwrap();
  let rrow = Regex::new(r"^rotate row y=(?P<y>.+) by (?P<n>.+)$").unwrap();

  let mut grid = [[false; WIDTH]; HEIGHT];

  for line in INPUT.lines() {
    if let Some(captures) = rect.captures(line) {
      let w = captures.name("w").unwrap().parse::<usize>().unwrap();
      let h = captures.name("h").unwrap().parse::<usize>().unwrap();
      for x in 0..w {
        for y in 0..h {
          grid[y][x] = true;
        }
      }
    } else if let Some(captures) = rcol.captures(line) {
      let x = captures.name("x").unwrap().parse::<usize>().unwrap();
      let n = captures.name("n").unwrap().parse::<usize>().unwrap();

      let old_grid = grid.clone();

      for y in 0..HEIGHT {
        grid[(y+n)%HEIGHT][x] = old_grid[y][x];
      }
    } else if let Some(captures) = rrow.captures(line) {
      let y = captures.name("y").unwrap().parse::<usize>().unwrap();
      let n = captures.name("n").unwrap().parse::<usize>().unwrap();

      let old_grid = grid.clone();

      for x in 0..WIDTH {
        grid[y][(x+n)%WIDTH] = old_grid[y][x];
      }

    } else { unimplemented!() }
  }

  viz(grid);

  let count = grid.iter().flat_map(|r| r.into_iter()).filter(|&&b| b).count();
  println!("{}", count);
}
