fn main() {
    let mut nums = std::fs::read_to_string("input.txt")
    .unwrap()
    .lines()
    .map(|l| l.parse::<u64>().unwrap())
    .collect::<Vec<_>>();

    nums.sort();

    let mut jolts = std::collections::HashMap::new();
    let mut series = Vec::new();

    let mut prev = 0;
    let mut series_count = 1;

    for num in nums {
        let diff = num - prev;
        jolts
            .entry(diff)
            .and_modify(|e| *e += 1)
            .or_insert(1);
        prev = num;

        if diff == 3 {
            series.push(series_count);
            series_count = 0;
        }
        series_count += 1;
    }

    series.push(series_count);

    jolts.entry(3).and_modify(|e| *e += 1);

    println!{"{:#?}", jolts};
    println!("Result: {}", jolts[&1] * jolts[&3]);

    println!("{:?}", series);
    println!("Combinations: {}", series.iter().map(|s| fun(*s)).fold(1, |s, f| s*f));
}

fn fun(n: u64) -> u64 {
    match n {
        1 => 1,
        2 => 1,
        3 => 2,
        4 => 4,
        5 => 7,
        _ => panic!()
    }
}