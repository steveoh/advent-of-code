// 3-bit computer with 3-bit instructions (0-7)
// 0: 000
// 1: 001
// 2: 010
// 3: 011
// 4: 100
// 5: 101
// 6: 110
// 7: 111
// 3 registers A, B, C
// 8 instructions which is a 3-bit number opcode
// 3-bit number after is the operand
// use a pointer to go through the instructions starting
// at 0 increasing by 2 except for jump instructions
// halt if tries to read past the end
// operands
// 0-3 is 0-3
// 4 is the value in the register A
// 5 is the value in the register B
// 6 is the value in the register C
// 7 will never appear
// instructions
// 0 is adv (division) A = floor(A/2^combo operand)
// 1 is bxl (bitwise) B = B xor literal operand
// 2 is bst (modulo) B = combo % 8
// 3 is jnz (jump) if A == 0. A != 0 pointer = operand skip double increase
// 4 is bxc bitwise B = B xor C move the pointer and ignore
// 5 is out (output) print combo % 8 joined by commas
// 6 is bdv (division) B = floor(A/2^combo operand)
// 7 is cdv (division) C = floor(A/2^combo operand)

#[derive(Debug)]
struct Instruction {
    opcode: u8,
    operand: u8,
}

fn get_combo(registers: &[isize], operand: u8) -> isize {
    match operand {
        0..=3 => operand as isize,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => unreachable!(),
    }
}

fn main() {
    let input = include_str!("../day-17.data");
    let mut registers = vec![0, 0, 0];
    let mut pointer = 0;
    let mut output = Vec::new();

    let (values, data) = input.split_once("\n\n").unwrap();
    for (i, line) in values.lines().enumerate() {
        let (_, value) = line.split_once(':').unwrap();
        registers[i] = value.trim().parse::<isize>().unwrap();
    }

    let (_, text) = data.split_once(':').unwrap();
    let instructions: Vec<Instruction> = text
        .trim()
        .split(',')
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .flatten()
        .collect::<Vec<u8>>()
        .chunks(2)
        .map(|chunk| Instruction {
            opcode: chunk[0],
            operand: chunk[1],
        })
        .collect();

    loop {
        if pointer >= instructions.len() {
            break;
        }

        let instruction = &instructions[pointer as usize];
        match instruction.opcode {
            0 => {
                registers[0] = registers[0] / 2isize.pow(instruction.operand as u32);
            }
            1 => {
                registers[1] = registers[1] ^ instruction.operand as isize;
            }
            2 => {
                registers[1] = get_combo(&registers, instruction.operand) % 8;
            }
            3 => {
                if registers[0] != 0 {
                    pointer = instruction.operand as usize / 2;
                    continue;
                }
            }
            4 => {
                registers[1] = registers[1] ^ registers[2];
                pointer += 1;
                continue;
            }
            5 => {
                output.push(get_combo(&registers, instruction.operand) % 8);
            }
            6 => {
                registers[1] =
                    registers[0] / 2isize.pow(get_combo(&registers, instruction.operand) as u32);
            }
            7 => {
                registers[2] =
                    registers[0] / 2isize.pow(get_combo(&registers, instruction.operand) as u32);
            }
            _ => unreachable!(),
        }

        pointer += 1;
    }
    let result = output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    // assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    println!("{}", result);
}
