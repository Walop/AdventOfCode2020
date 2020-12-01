use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    
    let lines = contents.lines();

    let mut read = Vec::new();

    let mut two_done = false;
    let mut three_done = false;

    for line in lines {
        let number = line.parse::<i32>().unwrap();
        for num in &read {
            if num + number == 2020 && !two_done {
                println!("Found {} and {}", num, number);
                println!("Result {}", num * number);
                two_done = true;
            }
            if num + number < 2020 {
                for num2 in &read {
                    if num + num2 + number == 2020 {
                        println!("Found {} and {} and {}", num, num2, number);
                        println!("Result {}", num * num2 * number);
                        three_done = true
                    }
                }
            }
            if two_done && three_done {
                break;
            }
        }
        read.push(number);
        if two_done && three_done {
            break;
        }
        println!("|{}|", line);
    }
}
