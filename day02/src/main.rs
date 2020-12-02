use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    
    let lines = contents.lines();

    let mut valid = 0;
    let mut valid2 = 0;

    for line in lines {
        let mut parts = line.split(" ");
        let range: Vec<usize> = parts.next().unwrap().split("-").map(|x| x.parse::<usize>().unwrap()).collect();
        let letter = parts.next().unwrap().chars().next().unwrap();
        let password = parts.next().unwrap();

        println!("{}-{} {}: {}", range[0], range[1], letter, password);

        let letter_count = password.matches(letter).count();

        println!("count: {}", letter_count);

        if letter_count >= range[0] && letter_count <= range[1] {
            valid += 1;
        }

        let place1 = password.chars().nth(range[0] - 1).unwrap();
        println!("char at place1: {}", place1);

        let place2 = password.chars().nth(range[1] - 1).unwrap();
        println!("char at place2: {}", place2);

        if (place1 == letter) ^ (place2 == letter) {
            valid2 += 1;
        }
    }

    println!("{} passwords were valid by first rule", valid);
    println!("{} passwords were valid by second rule", valid2);
}
