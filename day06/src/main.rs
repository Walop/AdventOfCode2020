fn main() {
    println!("-----START------");
    let contents = std::fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines = contents.lines();

    let mut any_yes = std::collections::HashSet::new();
    let mut any_yes_sets = Vec::new();

    let mut all_yes = std::collections::HashSet::new();
    let mut all_yes_sets = Vec::new();

    let mut set_initialized = false;

    for line in lines {
        if line == "" {
            all_yes_sets.push(all_yes);
            any_yes_sets.push(any_yes);

            all_yes = std::collections::HashSet::new();
            any_yes = std::collections::HashSet::new();

            set_initialized = false;

            continue;
        }
        let mut yes_answers = std::collections::HashSet::new();
        for c in line.chars() {
            yes_answers.insert(c);
        }
        if !set_initialized {
            any_yes = yes_answers.iter().copied().collect();
            all_yes = yes_answers.iter().copied().collect();
            set_initialized = true;
        }
        else {
            //println!("Union {} and {}", any_yes.iter().collect::<String>(), yes_answers.iter().collect::<String>());
            any_yes = any_yes.union(&yes_answers).cloned().collect();
            //println!("Union result {}\n", any_yes.iter().collect::<String>());
            //println!("Intersect {} and {}", all_yes.iter().collect::<String>(), yes_answers.iter().collect::<String>());
            all_yes = all_yes.intersection(&yes_answers).cloned().collect();
            //println!("Union result {}\n", any_yes.iter().collect::<String>());
        }
    }
    any_yes_sets.push(any_yes);
    all_yes_sets.push(all_yes);

    let mut sum = 0;

    for set in any_yes_sets {
        //println!("{}\n", set.iter().collect::<String>());
        sum += set.len();
    }

    println!("Sum of sets of yes: {}", sum);

    let mut all_yes_sum = 0;

    for set in all_yes_sets {
        //println!("{}\n", set.iter().collect::<String>());
        all_yes_sum += set.len();
    }

    println!("Sum of sets of all yes: {}", all_yes_sum);
}
