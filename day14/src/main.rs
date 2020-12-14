fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let mut and_mask = u64::MAX;
    let mut or_mask = u64::MIN;

    let mut mem = std::collections::HashMap::new();

    for line in content.lines() {
        match line.chars().skip(1).next().unwrap() {
            'a' => {
                or_mask = u64::MIN;
                or_mask |= u64::from_str_radix(&line.chars().skip(7).map(|c| if c == '1' { '1' } else { '0' }).collect::<String>(), 2).unwrap();
                and_mask = u64::MAX;
                and_mask &= u64::from_str_radix(&line.chars().skip(7).map(|c| if c == '0' { '0' } else { '1' }).collect::<String>(), 2).unwrap();
            },
            _ => {
                let addr = line.chars().skip_while(|c| !c.is_digit(10)).take_while(|c| c.is_digit(10)).collect::<String>().parse::<u64>().unwrap();
                let val = line.chars().skip_while(|c| c != &'=').skip_while(|c| !c.is_digit(10)).collect::<String>().parse::<u64>().unwrap();

                let m = mem.entry(addr).or_insert(0);
                *m = val & and_mask | or_mask;
            }
        }
    }

    //println!("{:?}", mem);
    println!("Sum of memory values {}", mem.values().sum::<u64>());

    let mut floating = Vec::new();

    mem.clear();

    for line in content.lines() {
        match line.chars().skip(1).next().unwrap() {
            'a' => {
                let mask = &line.chars().skip(7).collect::<Vec<_>>();
                or_mask = u64::MIN;
                or_mask |= u64::from_str_radix(&mask.iter().map(|c| if c == &'1' { '1' } else { '0' }).collect::<String>(), 2).unwrap();
                and_mask = u64::MAX;
                and_mask &= u64::from_str_radix(&mask.iter().skip(7).map(|c| if c == &'X' { '0' } else { '1' }).collect::<String>(), 2).unwrap();

                floating = mask.iter().rev().enumerate().filter(|(_,c)| c == &&'X').map(|(i,_)| 1 << i).collect::<Vec<_>>();
            },
            _ => {
                let addr = line.chars().skip_while(|c| !c.is_digit(10)).take_while(|c| c.is_digit(10)).collect::<String>().parse::<u64>().unwrap() & and_mask | or_mask;
                let val = line.chars().skip_while(|c| c != &'=').skip_while(|c| !c.is_digit(10)).collect::<String>().parse::<u64>().unwrap();

                for offs in generate_offsets(&floating) {
                    let m = mem.entry(addr + offs).or_insert(0);
                    *m = val;
                }
            }
        }
    }

    println!("Sum of memory values {}", mem.values().sum::<u64>());
}

fn generate_offsets(bits: &Vec<u64>) -> Vec<u64> {
    let mut res = Vec::new();
    for i in 0..1 << bits.len() {
        let mut temp = i;
        let mut index = 0;
        let mut sum = 0;
        while index < bits.len() {
            if temp & 1 == 1 {
                sum += bits[index];
            }
            temp >>= 1;
            index += 1;
        }
        res.push(sum);
    }
    return res;
}