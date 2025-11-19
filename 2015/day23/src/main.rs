static INPUT_FILE: &str = include_str!("../input.txt");

fn main() {
    let mut simulator = Simulator::new();

    while simulator.ip < INPUT_FILE.lines().count() {
        simulator.execute_instruction();
    }

    println!("Value in register b: {}", simulator.b);
}

struct Simulator {
    a: u32,
    b: u32,
    ip: usize,
}

impl Simulator {
    fn new() -> Self {
        Simulator { a: 0, b: 0, ip: 0 }
    }

    fn execute_instruction(&mut self) {
        let instruction = INPUT_FILE.lines().nth(self.ip).unwrap();
        let parts: Vec<&str> = instruction.split_whitespace().collect();

        match parts.as_slice() {
            ["hlf", reg] => {
                match *reg {
                    "a" => self.a /= 2,
                    "b" => self.b /= 2,
                    _ => panic!("Unknown register: {}", reg),
                }
            }
            ["tpl", reg] => {
                match *reg {
                    "a" => self.a *= 3,
                    "b" => self.b *= 3,
                    _ => panic!("Unknown register: {}", reg),
                }
            }
            ["inc", reg] => {
                match *reg {
                    "a" => self.a += 1,
                    "b" => self.b += 1,
                    _ => panic!("Unknown register: {}", reg),
                }
            }
            ["jmp", offset] => {
                let offset: isize = offset.parse().unwrap();
                if offset.is_negative() {
                    self.ip -= offset.wrapping_abs() as usize;
                } else {
                    self.ip += offset as usize;
                }
                return; // Early return
            }
            ["jie", reg, offset] => {
                let reg_value = match *reg {
                    "a," => self.a,
                    "b," => self.b,
                    _ => panic!("Unknown register: {}", reg),
                };
                if reg_value % 2 == 0 {
                    let offset: isize = offset.parse().unwrap();
                    if offset.is_negative() {
                        self.ip -= offset.wrapping_abs() as usize;
                    } else {
                        self.ip += offset as usize;
                    }
                    return; // Early return
                }
            }
            ["jio", reg, offset] => {
                let reg_value = match *reg {
                    "a," => self.a,
                    "b," => self.b,
                    _ => panic!("Unknown register: {}", reg),
                };
                if reg_value == 1 {
                    let offset: isize = offset.parse().unwrap();
                    if offset.is_negative() {
                        self.ip -= offset.wrapping_abs() as usize;
                    } else {
                        self.ip += offset as usize;
                    }
                    return; // Early return
                }
            }
            _ => panic!("Unknown instruction: {}", instruction),
        }

        self.ip += 1;
    }
    
}