// const INPUT: &'static str = "cpy 2 a
// tgl a
// tgl a
// tgl a
// cpy 1 a
// dec a
// dec a";
const INPUT: &'static str = "cpy a b
dec b
cpy a d
cpy 0 a
cpy b c
inc a
dec c
jnz c -2
dec d
jnz d -5
dec b
cpy b c
cpy c d
dec d
inc c
jnz d -2
tgl c
cpy -16 c
jnz 1 c
cpy 87 c
jnz 97 d
inc a
inc d
jnz d -2
inc c
jnz c -5";


fn main() {
  let instructions = INPUT.lines().map(|l| l.split(" ").collect::<Vec<_>>() ).collect::<Vec<_>>();
  let mut toggles = Vec::with_capacity(instructions.len());
  for _ in 0..instructions.len() { toggles.push(false) }

  let mut pc = 0;
  let mut registers = [12, 0, 0, 0];

  fn regindex(s: &str) -> usize {
    (s.bytes().next().unwrap() - b'a') as usize
  }

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
      }
      _ => unimplemented!(),
    }

    pc += 1;
  }

  println!("{:?}", registers);
}
