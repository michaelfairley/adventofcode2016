extern crate regex;

// const INPUT: &'static str = "cpy 41 a
// inc a
// inc a
// dec a
// jnz a 2
// dec a";

const INPUT: &'static str = "cpy 1 a
cpy 1 b
cpy 26 d
jnz c 2
jnz 1 5
cpy 7 c
inc d
dec c
jnz c -2
cpy a c
inc a
dec b
jnz b -2
cpy c b
dec d
jnz d -6
cpy 17 c
cpy 18 d
inc a
dec d
jnz d -2
dec c
jnz c -5";

fn main() {
  let cpy = regex::Regex::new(r"cpy (?P<src>.+) (?P<dst>\w)").unwrap();
  let inc = regex::Regex::new(r"inc (?P<reg>\w)").unwrap();
  let dec = regex::Regex::new(r"dec (?P<reg>\w)").unwrap();
  let jnz = regex::Regex::new(r"jnz (?P<reg>\w) (?P<diff>-?\d+)").unwrap();

  let instructions = INPUT.lines().collect::<Vec<_>>();
  let mut pc = 0;
  let mut registers = [0, 0, 1, 0];

  fn regindex(s: &str) -> usize {
    (s.bytes().next().unwrap() - b'a') as usize
  }

  while pc < instructions.len() {
    let instruction = instructions[pc];

    if let Some(captures) = cpy.captures(instruction) {
      let dst = &captures["dst"];
      let src = captures["src"].parse::<i32>().unwrap_or_else(|_|registers[regindex(&captures["src"])]);

      registers[regindex(dst)] = src;
    } else if let Some(captures) = inc.captures(instruction) {
      let reg = &captures["reg"];

      registers[regindex(reg)] += 1;
    } else if let Some(captures) = dec.captures(instruction) {
      let reg = &captures["reg"];

      registers[regindex(reg)] -= 1;
    } else if let Some(captures) = jnz.captures(instruction) {
      let reg = captures["reg"].parse::<i32>().unwrap_or_else(|_|registers[regindex(&captures["reg"])]);
      let diff = captures["diff"].parse::<i32>().unwrap();

      if reg != 0 {
        pc = (pc as i32 + diff - 1) as usize;
      }
    } else {
      unimplemented!();
    }


    pc += 1;
  }

  println!("{:?}", registers);
}
