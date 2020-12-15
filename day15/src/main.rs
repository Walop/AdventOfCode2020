fn main() {
    let start = std::time::Instant::now();

    let content = std::fs::read_to_string("input.txt").unwrap();

    let mut last_seen = std::collections::HashMap::new();
    let mut last_num = 0;

    for (i, num) in content.split(",").map(|n| n.parse::<usize>().unwrap()).enumerate() {
        last_seen.insert(num, i+1);
        last_num = num;
    }

    last_seen.remove(&last_num);
    println!("\nInitial {:?}", last_seen);

    let count = 30000000;

    let mut exp = 10;

    for i in last_seen.len()+1..count {
        let mut next_num = 0;
        if last_seen.contains_key(&last_num) {
            next_num = i - last_seen[&last_num];
        }
        
        *last_seen.entry(last_num).or_default() = i;
        last_num = next_num;

        if i == exp {
            println!("{}", exp);
            println!("Took {}ms", start.elapsed().as_millis());
            exp *= 10;
        }
    }

    println!("{}th number is {}", count, last_num);
    println!("Took {}ms", start.elapsed().as_millis());
}
