fn main() {
    let nums = std::fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let search_from_count = 25;
    
    let mut invalid_num: i64 = 0;

    for (i, num) in nums.iter().enumerate().skip(search_from_count) {
        if !find_pair_sum(num, i, &nums, search_from_count) {
            println!("Finding pair sum for {} failed", num);
            invalid_num = *num;
            break;
        }
    }

    for start in 0..nums.len() {
        if let Some(res) = find_sequence(&invalid_num, start, &nums) {
            let min = nums[start..=res].iter().max().unwrap();
            let max = nums[start..=res].iter().min().unwrap();
            println!("Min: {}, Max: {}, Sum: {}", min, max, min+max);
            break;
        }
    }
}

fn find_pair_sum(target: &i64, start: usize, nums: &Vec<i64>, search_from_count: usize) -> bool {
    let mut previous = std::collections::HashSet::new();

    for num in &nums[start-search_from_count..start] {
        if previous.contains(&(target-num)) {
            return true;
        }
        previous.insert(num);
    }
    false
}

fn find_sequence(target: &i64, start: usize, nums: &Vec<i64>) -> Option<usize> {
    let mut sum = 0;
    let mut i = start;
    while &sum < target {
        sum += nums[i];
        if &sum == target {
            return Some(i);
        }
        i += 1;
    }

    None
}