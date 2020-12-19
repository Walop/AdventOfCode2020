#[derive(Debug)]
struct Rule {
    match_char: Option<char>,
    refs: Option<Vec<i32>>,
    refs2: Option<Vec<i32>>
}

fn main() {
    println!("===============================================");
    let content = std::fs::read_to_string("input.txt").unwrap();

    let mut in_rules = true;
    let mut messages = Vec::new();
    let mut rules = std::collections::HashMap::new();

    for line in content.lines() {
        if in_rules && line.is_empty() {
            in_rules = false;
            continue;
        }
        if in_rules {
            let mut numbers = Vec::new();
            let mut num = "".to_string();
            let mut pipe = 0;
            let mut match_char = '\0';
            for c in line.chars() {
                if c == '|' {
                    pipe = numbers.len();
                }
                if c.is_ascii_digit() {
                    num.push(c);
                }
                else if c.is_alphabetic() {
                    match_char = c;
                }
                else if !num.is_empty() {
                    numbers.push(num.parse::<i32>().unwrap());
                    num = "".to_string();
                }
            }
            if numbers.len() == 0 {
                continue;
            }
            if !num.is_empty() {
                numbers.push(num.parse::<i32>().unwrap());
            }

            if match_char != '\0' {
                rules.insert(*numbers.first().unwrap(), Rule {match_char: Some(match_char), refs: None, refs2: None});
            }
            else {
                if pipe == 0 {
                    let refs = numbers[1..numbers.len()].to_vec();
                    rules.insert(numbers[0], Rule {match_char: None, refs: Some(refs), refs2: None});
                }
                else {
                    let refs = numbers[1..pipe].to_vec();
                    let refs2 = numbers[pipe..numbers.len()].to_vec();
                    rules.insert(numbers[0], Rule {match_char: None, refs: Some(refs), refs2: Some(refs2)});
                }
            }
        }
        else {
            messages.push(line);
        }
    }
    
    //println!("{:#?}", rules);
    //println!("{:?}", messages);

    let mut valid_count = 0;

    for message in messages.iter() {
        //println!("{}", message);
        let (offset, result) = resolve_rules(message, &rules, 0, 0, 0);
        //println!("Final offset {}, result {}", offset, result);
        if result && offset == message.len() {
            valid_count += 1;
        }
    }

    println!("Valid count: {}", valid_count);

    // NOT WORKING AFTER THIS
    rules.entry(8).and_modify(|r| *r = Rule {match_char: None, refs: Some(vec![42]), refs2: Some(vec![42, 8])});
    rules.entry(11).and_modify(|r| *r = Rule {match_char: None, refs: Some(vec![42,31]), refs2: Some(vec![42, 11, 31])});

    valid_count = 0;

    for message in messages.iter() {
        let (offset, result) = resolve_rules(message, &rules, 0, 0, 0);
        //println!("Final offset {}, result {}", offset, result);
        if result && offset == message.len() {
            println!("{}", message);
            valid_count += 1;
        }
    }

    println!("IN-Valid count: {}", valid_count);
}

fn resolve_rules(message: &str, rules: &std::collections::HashMap<i32, Rule>, char_idx: usize, rule_nmb: i32, rec: usize) -> (usize, bool) {
    print!("Rule {} -> ", rule_nmb);
    if let Some(match_char) = rules[&rule_nmb].match_char {
        match message.chars().nth(char_idx) {
            Some(c) => {
                println!("Matching {} with {} at index {}: {}", match_char, c, char_idx, match_char == c);
                return (1, match_char == c);
            },
            None => {
                println!("Ran out of message at index {}", char_idx);
                return (0, false);
            }
        }
    }
    if let Some(refs) = &rules[&rule_nmb].refs {
        let mut char_offset = 0;
        //println!("{:?}", refs);
        let mut final_result = true;
        for rule_ref in refs {
            let (offset, result) = resolve_rules(&message, rules, char_idx + char_offset, *rule_ref, rec+1);
            if !result {
                final_result = false;
                break;
            }
            char_offset += offset;
        }
        if final_result {
            println!("First matched, short circuit");
            println!("Rec {} Rule {} true", rec, rule_nmb);
            return (char_offset, final_result);
        }
        if let Some(refs2) = &rules[&rule_nmb].refs2 {
            char_offset = 0;
            for rule_ref in refs2 {
                let (offset, result) = resolve_rules(&message, rules, char_idx + char_offset, *rule_ref, rec+1);
                if !result {
                    return (char_offset, false);
                }
                char_offset += offset;
            }
            println!("Second matched");
            println!("Rule {} true", rule_nmb);
            return (char_offset, true);
        }
        return (0, false);
    }
    println!("None matched");
    (0,false)
}