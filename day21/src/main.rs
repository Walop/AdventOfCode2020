fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let mut ingredient_allergens = std::collections::HashMap::new();
    let mut ingredient_instances = Vec::new();

    for line in content.lines() {
        let split = line.split(' ');
        let mut ingredient = true;
        let mut ingredients = std::collections::HashSet::new();
        let mut allergens = Vec::new();
        for word in split {
            if word == "(contains" {
                ingredient = false;
            } else if ingredient {
                ingredients.insert(word.to_string());
                ingredient_instances.push(word.to_string());
            } else {
                allergens.push(word.chars().take(word.len() - 1).collect::<String>());
            }
        }
        for all in allergens {
            let ia = ingredient_allergens.entry(all).or_insert(
                ingredients
                    .iter()
                    .cloned()
                    .collect::<std::collections::HashSet<_>>(),
            );
            let ing = ingredients.iter().cloned().collect();

            *ia = ia.intersection(&ing).cloned().collect();
        }
    }

    //println!("{:?}", ingredient_allergens);

    let mut with_allergens = std::collections::HashSet::new();

    for ing in ingredient_allergens.iter().map(|(_, v)| v).flatten() {
        with_allergens.insert(ing);
    }

    //println!("{:?}", with_allergens);

    let without_count = ingredient_instances
        .clone()
        .iter()
        .filter(|ing| !with_allergens.contains(ing))
        .count();

    println!("{}", without_count);

    let mut tuples = ingredient_allergens
        .iter()
        .map(|(k, v)| (k, v))
        .collect::<Vec<_>>();
    tuples.sort_by_key(|t| t.1.len());

    let mut ing_with_al = Vec::new();
    let mut known_ing = Vec::new();

    for (al, ing) in tuples {
        let next_known = ing.iter().find(|i| !known_ing.contains(i)).unwrap();
        known_ing.push(next_known);
        ing_with_al.push((next_known, al));
    }

    ing_with_al.sort_by_key(|k| k.1);

    println!("{:?}", ing_with_al);

    println!(
        "{}",
        ing_with_al
            .iter()
            .map(|(ing, _)| *ing)
            .fold(String::new(), |acc, ing| acc + "," + ing)
    );
}
