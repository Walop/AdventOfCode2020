fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let mut lines = content.lines();
    
    let arrival = lines.next().unwrap().parse::<i64>().unwrap();
    let departures = lines.next().unwrap()
        .split(',')
        .enumerate()
        .filter(|(_,s)| s != &"x")
        .map(|(i,s)| (i, s.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();

    println!("{}\n{:?}", arrival, departures);

    let min_wait_time = departures.iter().enumerate()
        .map(|(i,(_,d))| (i, d - arrival % d))
        .min_by_key(|d| d.1)
        .unwrap();

    println!("Bus ID {}, wait time {}, result {}\n", departures[min_wait_time.0].1, min_wait_time.1, departures[min_wait_time.0].1 * min_wait_time.1);

    let mut t = 0;
    
    let mut cycle = departures[0].1;

    let mut sorted = departures[1..departures.len()].iter().collect::<Vec<_>>();
    sorted.sort_by_key(|d| d.1);

    let mut exp = 2;

    for d in sorted.iter().rev() {
        println!("Current bus: {:?}", d);
        println!("Cycle: {}", cycle);
        let m = d.0 as i64 % d.1;
        println!("Mod {}", m);
        while d.1 - t % d.1 != m as i64 {
            t += cycle;
            if t > exp {
                println!("Current time = {}", exp);
                exp *= 2;
            }
        }
        cycle *= d.1;
    }

    println!("Result: {}", t);
}
