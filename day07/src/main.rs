fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    
    let lines = contents.lines();

    let mut rules: std::collections::HashMap<String, Vec<(i32, String)>> = std::collections::HashMap::new();
    let mut rules2: std::collections::HashMap<String, Vec<(i32, String)>> = std::collections::HashMap::new();

    for line in lines {
        let mut split = line.split(" ").collect::<Vec<&str>>();
        let container = split.drain(..2).collect::<Vec<&str>>().concat();
        split.drain(..2);
        let mut counter = 0;
        let mut entry = (0,"");
        let mut xyz = Vec::new();
        for x in split {
            match counter {
                0 => {
                    entry.0 = match x.parse::<i32>() {
                        Ok(y) => y,
                        Err(_) => 0
                    };
                    counter += 1;
                }
                1 => {
                    entry.1 = x;
                    counter += 1;
                }
                2 => {
                    let contained = [entry.1, x].concat();
                    rules.entry(contained.clone()).or_default().push((entry.0, container.clone()));
                    xyz.push((entry.0, contained));
                    counter += 1;
                }
                3 => counter = 0,
                _ => println!("Should not come here")
            }
        }
        rules2.insert(container.clone(), xyz);
    }

    /*for (contained, containers) in &rules {
        for container in containers {
            println!("{} contains {} {}", container.1, container.0, contained);
        }
    }*/

    let set = outer_bags(&rules, "shinygold");

    println!("Types of outmost bags {}", set.len());

    println!("Sum of inner bags {}", inner_bags(&rules2, "shinygold"));
}

fn outer_bags(rules: &std::collections::HashMap<String, Vec<(i32, String)>>, container: &str) -> std::collections::HashSet<String> {
    if !rules.contains_key(container) {
        return std::collections::HashSet::new();
    }
    let mut set = std::collections::HashSet::new();
    for c in rules[container].iter() {
        set.insert(c.1.clone());
        set.extend(outer_bags(&rules, &c.1));
    }
    set
}

fn inner_bags(rules: &std::collections::HashMap<String, Vec<(i32, String)>>, container: &str) -> i32 {
    let mut sum = 0;
    if !rules.contains_key(container) {
        return 1;
    }
    for c in rules[container].iter() {
        sum += c.0;
        sum += c.0 * inner_bags(&rules, &c.1);
    }
    sum
}