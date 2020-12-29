fn main() {
    let keys: Vec<i64> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let mut subject = 7;
    let mut loops = 0;

    while subject != keys[0] {
        loops += 1;
        subject = subject * 7 % 20201227;
    }

    subject = keys[1];

    for _ in 0..loops {
        subject = subject * keys[1] % 20201227;
    }

    println!("Final key: {}", subject);
}
