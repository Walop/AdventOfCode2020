fn main() {
    let initial = "315679824";

    let input: Vec<i32> = initial
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32 - 1)
        .collect();
    let mut cups = vec![0; input.len()];

    for i in 0..input.len() {
        cups[input[i] as usize] = input[(i + 1) % input.len()];
    }

    run_game(&mut cups, input[0], 100);

    let mut current = cups[0];

    let result1 = (0..input.len() - 1).fold(0, |acc, _| {
        let val = (acc * 10) + (current + 1);
        current = cups[current as usize];
        val
    });

    println!("Result 1: {}", result1);

    let mut input2 = input.clone();
    for c in input.len()..1_000_000 {
        input2.push(c as i32);
    }
    let mut more_cups = vec![0; 1_000_000];

    for i in 0..1_000_000 {
        more_cups[input2[i] as usize] = input2[(i + 1) % 1_000_000];
    }

    run_game(&mut more_cups, input[0], 10_000_000);

    let first = more_cups[0] + 1;
    let second = more_cups[more_cups[0] as usize] + 1;

    let result2 = first as u64 * second as u64;

    println!("Result 2: {}", result2);
}

fn run_game(cups: &mut Vec<i32>, start: i32, rounds: u32) {
    let cup_count = cups.len() as i32;

    let mut current: i32 = start;

    for _ in 0..rounds {
        let next_1 = cups[current as usize];
        let next_2 = cups[next_1 as usize];
        let next_3 = cups[next_2 as usize];

        let mut target = (current - 1).rem_euclid(cup_count);
        while [next_1, next_2, next_3].contains(&target) {
            target = (target - 1).rem_euclid(cup_count);
        }

        cups.swap(current as usize, next_3 as usize);
        cups.swap(next_3 as usize, target as usize);

        current = cups[current as usize];
    }
}
