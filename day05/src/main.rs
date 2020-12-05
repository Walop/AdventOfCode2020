fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines = contents.lines();

    let mut seat_ids = Vec::new();

    for line in lines {
        let mut rmin = 0;
        let mut rmax = 128;
        let mut cmin = 0;
        let mut cmax = 8;
        for c in line.chars() {
            match c {
                'F' => {
                    rmax -= (rmax - rmin) / 2;
                }
                'B' => {
                    rmin += (rmax - rmin) / 2;
                }
                'L' => {
                    cmax -= (cmax - cmin) / 2;
                }
                'R' => {
                    cmin += (cmax - cmin) / 2;
                }
                _ => println!("Unknown char {}", c)
            }
        }
        //println!("Row {}, Column {}, ID: {}", rmin, cmin, rmin * 8 + cmin);
        seat_ids.push(rmin * 8 + cmin);
    }

    seat_ids.sort();

    println!("Highest id was {}", seat_ids.last().unwrap());

    let mut last_id = seat_ids.first().unwrap();
    
    for id in &seat_ids {
        if (id - last_id) > 1 {
            println!("My seat ID {}", last_id + 1);
            break;
        }
        last_id = id;
    }
}
