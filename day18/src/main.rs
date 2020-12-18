fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let mut expression = Vec::new();
    let mut temp = Vec::new();

    let mut sum = 0;

    for line in content.lines() {
        for c in line.chars() {
            match c {
                '0'..='9' => expression.push(c),
                '(' => temp.push(c),
                ')' => {
                    while !temp.is_empty() && *temp.last().unwrap() != '(' {
                        expression.push(temp.pop().unwrap());
                    }
                    if *temp.last().unwrap() == '(' {
                        temp.pop();
                    }
                },
                '*'..='+' => {
                    while !temp.is_empty() && prec(&c) <= prec(temp.last().unwrap()) {
                        expression.push(temp.pop().unwrap());
                    }
                    temp.push(c);
                },
                _ => {}
            }
        }
        while !temp.is_empty() {
            expression.push(temp.pop().unwrap());
        }

        sum += eval(&expression);
    }

    println!("Sum of expressions is {}", sum);

    sum = 0;

    for line in content.lines() {
        for c in line.chars() {
            match c {
                '0'..='9' => expression.push(c),
                '(' => temp.push(c),
                ')' => {
                    while !temp.is_empty() && *temp.last().unwrap() != '(' {
                        expression.push(temp.pop().unwrap());
                    }
                    if *temp.last().unwrap() == '(' {
                        temp.pop();
                    }
                },
                '*'..='+' => {
                    while !temp.is_empty() && prec2(&c) <= prec2(temp.last().unwrap()) {
                        expression.push(temp.pop().unwrap());
                    }
                    temp.push(c);
                },
                _ => {}
            }
        }
        while !temp.is_empty() {
            expression.push(temp.pop().unwrap());
        }

        sum += eval(&expression);
    }

    println!("Sum of expressions with + precedence is {}", sum);
}

fn prec(op: &char) -> i8 {
    return match op {
        '*' => 1,
        '+' => 1,
        _ => 0
    };
}

fn prec2(op: &char) -> i8 {
    return match op {
        '*' => 1,
        '+' => 2,
        _ => 0
    };
}

fn eval(exp: &Vec<char>) -> i64 {
    let mut temp = Vec::new();
    let mut expression = exp.iter().rev().collect::<Vec<_>>();
    while !expression.is_empty() {
        //print!("{}", expression.last().unwrap());
        if expression.last().unwrap().is_ascii_digit() {
            temp.push(*expression.pop().unwrap() as i64 - '0' as i64);
        }
        else {
            let o1 = temp.pop().unwrap();
            let o2 = temp.pop().unwrap();
            match expression.pop().unwrap() {
                '+' => temp.push(o1 + o2),
                '*' => temp.push(o1 * o2),
                _ => panic!()
            }
        }
    }

    temp.pop().unwrap()
}