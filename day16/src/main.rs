fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let mut rules = Vec::new();
    let mut tickets = Vec::new();
    let mut rules_section = true;

    for line in content.lines() {
        if rules_section && line.is_empty() {
            rules_section = false;
            continue;
        }
        let mut numbers = Vec::new();
        let mut num = "".to_string();
        for c in line.chars() {
            if c.is_ascii_digit() {
                num.push(c);
            }
            else if !num.is_empty() {
                numbers.push(num.parse::<i32>().unwrap());
                num = "".to_string();
            }
        }
        if numbers.len() == 0 {
            continue;
        }
        numbers.push(num.parse::<i32>().unwrap());

        if rules_section {
            rules.push(numbers);
        }
        else {
            tickets.push(numbers);
        }
    }

    let mut invalid_numbers = Vec::new();
    let mut valid_tickets = Vec::new();

    for ticket in &tickets {
        let mut valid_nums = 0;
        for num in ticket {
            let mut invalid_count = 0;
            for rule in &rules {
                if !((num >= &rule[0] && num <= &rule[1]) || (num >= &rule[2] && num <= &rule[3])) {
                    invalid_count += 1;
                }
            }
            if invalid_count == rules.len() {
                invalid_numbers.push(*num);
            }
            else {
                valid_nums += 1;
            }
        }
        if valid_nums == ticket.len() {
            valid_tickets.push(ticket);
        }
    }

    println!("Sum of invalid numbers {}", invalid_numbers.iter().sum::<i32>());
    println!("Valid tickets {}", valid_tickets.len());

    let mut matching_fields = Vec::new();

    for (r, rule) in rules.iter().enumerate() {
        let mut all_match = Vec::new();
        for i in 0..valid_tickets[0].len() {
            let mut valid = true;
            for ticket in &valid_tickets {
                let num = ticket[i];
                if !((num >= rule[0] && num <= rule[1]) || (num >= rule[2] && num <= rule[3])) {
                    valid = false;
                    break;
                }
            }
            if valid {
                all_match.push(i);
            }
        }
        matching_fields.push((r, all_match));
    }

    //println!("{:?}", matching_fields);

    let mut used = Vec::new();
    let mut mapping = Vec::new();

    matching_fields.sort_by_key(|m| m.1.len());

    for (i, field) in matching_fields {
        let left = field.iter().position(|x| !used.contains(x)).unwrap();
        used.push(field[left]);
        mapping.push((i, field[left]));
    }

    //println!("{:?}", mapping);

    let mut product = 1;

    for (r, f) in mapping {
        println!("{}, {:?}, {}", r, rules[r], &tickets[0][f]);
        if r < 6 {
            product *= tickets[0][f] as i64;
        }
    }

    println!("{}", product);
}
