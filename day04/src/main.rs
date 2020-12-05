fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let passport_data = contents.trim().split("\r\n\r\n");

    let mut passports = Vec::new();

    for data_row in passport_data {
        let one_line = data_row.replace("\n", " ");
        //println!("{}", one_line);
        let fields = one_line.split(" ");
        let mut map = std::collections::HashMap::new();
        for field in fields {
            let mut kv = field.split(":");
            map.insert(kv.next().unwrap().to_string(), kv.next().unwrap().to_string());
        }
        passports.push(map);
    }

    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut valid_count = 0;
    let mut valid_count2 = 0;

    for passport in passports {
        let mut found_fields = 0;
        let mut valid = true;
        for (k, v) in passport.iter() {
            //print!("{}: {} ", k, v);
            if required.contains(&k.as_str()) {
                found_fields += 1;
            }
            valid = valid && validate_field(k, v.trim());
            //println!("{}:{} {}", k, v, valid);
        }
        //println!("\n");
        if found_fields == required.len() {
            valid_count += 1;
            if valid {
                valid_count2 += 1;
            }
        }
    }

    println!("Valid passport count: {}", valid_count);
    println!("Valid fields passport count: {}", valid_count2);
}

fn validate_field(key: &str, value: &str) -> bool {
    match key {
        "byr" => {
            match value.parse::<i32>() {
                Ok(byr) => byr >= 1920 && byr <= 2002,
                Err(_) => {
                    println!("Could not parse byr {}", value);
                    false
                }
            }
        }
        "iyr" => {
            match value.parse::<i32>() {
                Ok(iyr) => iyr >= 2010 && iyr <= 2020,
                Err(_) => {
                    println!("Could not parse iyr {}", value);
                    false
                }
            }
            
        }
        "eyr" => {
            match value.parse::<i32>() {
                Ok(eyr) => eyr >= 2020 && eyr <= 2030,
                Err(_) => {
                    println!("Could not parse eyr {}", value);
                    false
                }
            }
        }
        "hgt" => {
            let len = value.chars().count();
            match &value[len-2..] {
                "cm" => {
                    match value[..len-2].parse::<i32>() {
                        Ok(i) => i >= 150 && i <= 193,
                        Err(_) => {
                            println!("Could not parse cm hgt {}", value);
                            false
                        }
                    }
                },
                "in" => {
                    match value[..len-2].parse::<i32>() {
                        Ok(i) => i >= 59 && i <= 76,
                        Err(_) => {
                            println!("Could not parse in hgt {}", value);
                            false
                        }
                    }
                }
                _ => {
                    println!("Invalid hgt {}", value);
                    false
                }
            }
        }
        "hcl" => {
            if value.chars().count() != 7 {
                println!("Invalid hcl length {}", value);
                return false;
            }
            if &value[0..1] != "#" {
                println!("Invalid hcl first char {}", value);
                return false;
            }
            match i64::from_str_radix(value.chars().next().map(|c| &value[c.len_utf8()..]).unwrap(), 16) {
                Ok(_) => true,
                Err(_) => {
                    println!("Could not parse hcl {}", value);
                    false
                }
            }
        }
        "ecl" => {
            let valid = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            if !valid.contains(&value) {
                println!("Invalid ecl {}", value);
                return false
            }
            true
        }
        "pid" => {
            if value.chars().count() != 9 {
                println!("Invalid pid length {}", value);
                return false;
            }
            match value.parse::<i32>() {
                Ok(_) => true,
                Err(_) => {
                    println!("Could not parse pid {}", value);
                    false
                }
            }
        }
        "cid" => true,
        _ => {
            println!("Key {} did not match", key);
            false
        }
    }
}