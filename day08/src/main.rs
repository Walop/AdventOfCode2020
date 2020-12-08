#[derive(Clone)]
struct Instruction {
    instruction: String,
    param: i32
}

fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    
    let lines = contents.lines();

    let mut program = Vec::new();

    for line in lines {
        let mut split = line.split(" ");
        let i = split.next().unwrap().to_string();
        let p = split.next().unwrap().parse::<i32>().unwrap();
        program.push(Instruction { instruction: i, param: p});
    }

    let (_, acc) = run_program(&program);

    println!("Acc before loop {}", acc);

    let mut switch = program.iter().enumerate().filter(|(_, inst)| inst.instruction != "acc").map(|(i, _)| i).rev().collect::<Vec<usize>>();

    loop {
        let mut program2 = program.clone();
        let switch_instruction = switch.pop().unwrap();
        println!("Switching instruction at {}", switch_instruction);
        if program2[switch_instruction].instruction == "nop" {
            program2[switch_instruction].instruction = "jmp".to_string();
        }
        else {
            program2[switch_instruction].instruction = "nop".to_string();
        }
        let (result, acc) = run_program(&program2);
        if result {
            println!("Acc after finishing program {}", acc);
            break;
        }
    }
}

fn run_program(program: &std::vec::Vec<Instruction>) -> (bool, i32) {
    let mut acc = 0;
    let mut program_counter = 0;
    let mut visited_lines = std::collections::HashSet::new();

    while program_counter < program.len() as i32 {

        if visited_lines.contains(&program_counter) {
            println!("Program looped at {}", program_counter);
            return (false, acc);
        }

        visited_lines.insert(program_counter);

        let line = &program[program_counter as usize];

        match line.instruction.as_str() {
            "nop" => program_counter += 1,
            "acc" => {
                acc += line.param;
                program_counter += 1;
            },
            "jmp" => program_counter += line.param,
            _ => panic!()
        }
    }

    (true, acc)
}
