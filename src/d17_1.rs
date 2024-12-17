struct Computer {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    instruction_pointer: usize,
    program: Vec<i64>,
    outputs: Vec<i64>,
}

impl Computer {
    fn new(program: Vec<i64>, reg_a: i64, reg_b: i64, reg_c: i64) -> Self {
        Computer {
            register_a: reg_a,
            register_b: reg_b,
            register_c: reg_c,
            instruction_pointer: 0,
            program,
            outputs: Vec::new(),
        }
    }

    fn get_combo_operand(&self, operand: i64) -> i64 {
        match operand {
            0..=3 => operand,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid combo operand: {}", operand),
        }
    }

    fn run(&mut self) {
        while self.instruction_pointer < self.program.len() {
            let opcode = self.program[self.instruction_pointer];
            let operand = self.program[self.instruction_pointer + 1];

            match opcode {
                0 => {
                    // adv
                    let power = self.get_combo_operand(operand);
                    self.register_a /= 2_i64.pow(power as u32);
                }
                1 => {
                    // bxl
                    self.register_b ^= operand;
                }
                2 => {
                    // bst
                    self.register_b = self.get_combo_operand(operand) % 8;
                }
                3 => {
                    // jnz
                    if self.register_a != 0 {
                        self.instruction_pointer = operand as usize;
                        continue;
                    }
                }
                4 => {
                    // bxc
                    self.register_b ^= self.register_c;
                }
                5 => {
                    // out
                    let value = self.get_combo_operand(operand) % 8;
                    self.outputs.push(value);
                }
                6 => {
                    // bdv
                    let power = self.get_combo_operand(operand);
                    self.register_b = self.register_a / 2_i64.pow(power as u32);
                }
                7 => {
                    // cdv
                    let power = self.get_combo_operand(operand);
                    self.register_c = self.register_a / 2_i64.pow(power as u32);
                }
                _ => panic!("Invalid opcode: {}", opcode),
            }

            self.instruction_pointer += 2;
        }
    }

    fn get_output(&self) -> String {
        self.outputs
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}
#[test]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("d17.in");
    // Example from the problem
    let program = input
        .split("\n")
        .skip_while(|p| !p.starts_with("Program: "))
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut computer = Computer::new(
        program,
        input
            .split("\n")
            .next()
            .unwrap()
            .strip_prefix("Register A: ")
            .unwrap()
            .parse()
            .unwrap(),
        0,
        0,
    );
    computer.run();
    println!("Output: {}", computer.get_output());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_program() {
        let program = vec![0, 1, 5, 4, 3, 0];
        let mut computer = Computer::new(program, 729, 0, 0);
        computer.run();
        assert_eq!(computer.get_output(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_other_examples() {
        // Test case: If register C contains 9, the program 2,6 would set register B to 1
        let mut computer = Computer::new(vec![2, 6], 0, 0, 9);
        computer.run();
        assert_eq!(computer.register_b, 1);

        // Test case: If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2
        let mut computer = Computer::new(vec![5, 0, 5, 1, 5, 4], 10, 0, 0);
        computer.run();
        assert_eq!(computer.get_output(), "0,1,2");

        // Test case: If register B contains 29, the program 1,7 would set register B to 26
        let mut computer = Computer::new(vec![1, 7], 0, 29, 0);
        computer.run();
        assert_eq!(computer.register_b, 26);

        // Test case: If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354
        let mut computer = Computer::new(vec![4, 0], 0, 2024, 43690);
        computer.run();
        assert_eq!(computer.register_b, 44354);
    }
}
