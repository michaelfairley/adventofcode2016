const INPUT: &'static str = "cpy a d
cpy 7 c
cpy 365 b
inc d
dec b
jnz b -2
dec c
jnz c -5
cpy d a
jnz 0 0
cpy a b
cpy 0 a
cpy 2 c
jnz b 2
jnz 1 6
dec b
dec c
jnz c -4
inc a
jnz 1 -7
cpy 2 b
jnz c 2
jnz 1 4
dec b
dec c
jnz 1 -4
jnz 0 0
out b
jnz a -19
jnz 1 -21";


fn main() {
  let instructions = INPUT.lines().map(|l| l.split(" ").collect::<Vec<_>>() ).collect::<Vec<_>>();
  let mut toggles = Vec::with_capacity(instructions.len());
  for _ in 0..instructions.len() { toggles.push(false) }


  fn regindex(s: &str) -> usize {
    (s.bytes().next().unwrap() - b'a') as usize
  }

  'aaa: for a in 0.. {
    let mut pc = 0;
    let mut registers = [a, 0, 0, 0];
    let mut output = vec![1];

    while pc < instructions.len() {
      let args = &instructions[pc];
      let toggle = toggles[pc];

      match (args[0], toggle) {
        ("cpy", false) | ("jnz", true) => {
          let src = args[1].parse::<i32>().unwrap_or_else(|_|registers[regindex(&args[1])]);
          let dst = args[2];

          registers[regindex(dst)] = src;
        },
        ("inc", false) | ("dec", true) | ("tgl", true) => {
          let reg = args[1];

          registers[regindex(reg)] += 1;
        },
        ("dec", false) | ("inc", true) => {
          let reg = args[1];

          registers[regindex(reg)] -= 1;
        },
        ("jnz", false) | ("cpy", true) => {
          let reg = args[1].parse::<i32>().unwrap_or_else(|_|registers[regindex(&args[1])]);
          let diff = args[2].parse::<i32>().unwrap_or_else(|_|registers[regindex(&args[2])]);

          if reg != 0 {
            pc = (pc as i32 + diff - 1) as usize;
          }
        },
        ("tgl", false) => {
          let reg = args[1];
          let index = pc + registers[regindex(reg)] as usize;

          if index < toggles.len() {
            toggles[index] = !toggles[index];
          }
        },
        ("out", false) => {
          let reg = args[1].parse::<i32>().unwrap_or_else(|_|registers[regindex(&args[1])]);

          let prev = *output.last().unwrap();
          if 1 - prev != reg {
            continue 'aaa;
          } else {
            output.push(reg);
            if output.len() > 100 {
              println!("{}", a);
              break 'aaa;
            }
          }
        },
        _ => unimplemented!(),
      }

      pc += 1;
    }
  }
}
