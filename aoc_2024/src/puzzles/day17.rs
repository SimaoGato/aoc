#[test]
fn test() {
    solve(String::from(
        "Register A: 2024
    Register B: 0
    Register C: 0

    Program: 0,3,5,4,3,0",
    ));
}

pub fn solve(data: String) {
    let mut registers = [0i64; 3];
    let mut program = Vec::new();

    for line in data.lines() {
        if let Some((name, value)) = line.split_once(":") {
            let value = value.trim();
            if name.trim() == "Program" {
                program = value
                    .split(',')
                    .map(|x| x.trim().parse::<u8>().unwrap())
                    .collect();
            } else {
                let value = value.parse::<i64>().unwrap();
                match name.trim() {
                    "Register A" => registers[0] = value,
                    "Register B" => registers[1] = value,
                    "Register C" => registers[2] = value,
                    _ => {}
                }
            }
        }
    }

    let part1_output = run_program(&mut registers, &program);
    println!("Part 1: {}", part1_output);

    if recursive_solve(&program, program.len() - 1, 0) {
        return;
    }
}

fn run_program(registers: &mut [i64; 3], program: &[u8]) -> String {
    let mut ip = 0;
    let mut output = Vec::new();

    while ip < program.len() {
        let opcode = program[ip];
        let operand = program[ip + 1];
        execute_instruction(opcode, operand, registers, &mut ip, &mut output);
    }

    return output
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",");
}

fn execute_instruction(
    opcode: u8,
    operand: u8,
    registers: &mut [i64; 3],
    ip: &mut usize,
    output: &mut Vec<i64>,
) {
    match opcode {
        0 => registers[0] /= 2_i64.pow(get_combo_value(operand, registers) as u32),
        1 => registers[1] ^= operand as i64,
        2 => registers[1] = (get_combo_value(operand, registers) % 8) as i64,
        3 => {
            if registers[0] != 0 {
                *ip = operand as usize;
                return;
            }
        }
        4 => registers[1] ^= registers[2],
        5 => output.push((get_combo_value(operand, registers) % 8) as i64),
        6 => registers[1] = registers[0] / 2_i64.pow(get_combo_value(operand, registers) as u32),
        7 => registers[2] = registers[0] / 2_i64.pow(get_combo_value(operand, registers) as u32),
        _ => (),
    }
    *ip += 2;
}

fn recursive_solve(program: &[u8], pos: usize, a: usize) -> bool {
    if pos == usize::MAX {
        println!("Part 2: {}", a);
        return true;
    }

    for d in 0..8 {
        let candidate_a = (a << 3) | d;

        if let Some(w) = simulate_until_output(candidate_a, program) {
            if w == program[pos] as usize {
                if recursive_solve(program, pos.wrapping_sub(1), candidate_a) {
                    return true;
                }
            }
        }
    }

    return false;
}

fn simulate_until_output(a: usize, program: &[u8]) -> Option<usize> {
    let mut registers = [a as i64, 0, 0];
    let mut ip = 0;

    while ip < program.len() {
        let opcode = program[ip];
        let operand = program[ip + 1];
        let mut output = Vec::new();

        execute_instruction(opcode, operand, &mut registers, &mut ip, &mut output);

        if !output.is_empty() {
            return Some(output[0] as usize);
        }
    }

    return None;
}

fn get_combo_value(operand: u8, registers: &[i64; 3]) -> u64 {
    return match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => registers[0] as u64,
        5 => registers[1] as u64,
        6 => registers[2] as u64,
        _ => panic!("Invalid combo operand"),
    };
}
