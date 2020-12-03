fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    
    let lines = contents.lines();

    let mut map = Vec::new();

    for line in lines {
        let chars: Vec<bool> = line.chars().map(|c| c == '#').collect();
        map.push(chars);
    }

    let hit_1_1 = check_slope(&map, 1, 1);
    println!("Slope 1-1 hit trees {} times", hit_1_1);

    let hit_3_1 = check_slope(&map, 3, 1);
    println!("Slope 3-1 hit trees {} times", hit_3_1);

    let hit_5_1 = check_slope(&map, 5, 1);
    println!("Slope 5-1 hit trees {} times", hit_5_1);

    let hit_7_1 = check_slope(&map, 7, 1);
    println!("Slope 7-1 hit trees {} times", hit_7_1);

    let hit_1_2 = check_slope(&map, 1, 2);
    println!("Slope 1-2 hit trees {} times", hit_1_2);

    println!("Answer: {}", hit_1_1 * hit_3_1 * hit_5_1 * hit_7_1 * hit_1_2);
}

fn check_slope(map: &Vec<Vec<bool>>, right: usize, down: usize) -> i64 {
    let width = map[0].len();

    let mut hit = 0;

    for (i, row) in map.iter().enumerate() {
        if i % down == 0 {
            if row[i * right / down % width] {
                hit += 1;
            }
        }
    }
    return hit;
}