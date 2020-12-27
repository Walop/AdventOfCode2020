fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let mut initial_deck1 = std::collections::VecDeque::new();
    let mut initial_deck2 = std::collections::VecDeque::new();

    let mut fill_deck1 = true;

    for line in content.lines() {
        if line.is_empty() {
            fill_deck1 = false;
        } else if fill_deck1 {
            if let Ok(num) = line.parse::<u8>() {
                initial_deck1.push_back(num);
            }
        } else {
            if let Ok(num) = line.parse::<u8>() {
                initial_deck2.push_back(num);
            }
        }
    }

    let mut deck1 = initial_deck1.clone();
    let mut deck2 = initial_deck2.clone();

    while !deck1.is_empty() && !deck2.is_empty() {
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    let deck1_value = deck1
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, card)| acc + (i + 1) * *card as usize);
    let deck2_value = deck2
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, card)| acc + (i + 1) * *card as usize);

    println!("Deck 1 value {}", deck1_value);
    println!("Deck 2 value {}", deck2_value);

    deck1 = initial_deck1.clone();
    deck2 = initial_deck2.clone();

    println!("Start recursive");

    let mut configs1 = std::collections::HashSet::new();
    let mut configs2 = std::collections::HashSet::new();

    let mut round = 0;

    while !deck1.is_empty() && !deck2.is_empty() {
        round += 1;
        // println!("Game 1, round {}", round);
        // println!("{:?}", deck1);
        // println!("{:?}", deck2);

        if !configs1.insert(deck1.clone()) || !configs2.insert(deck2.clone()) {
            break;
        }

        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        let result;
        if deck1.len() >= card1 as usize && deck2.len() >= card2 as usize {
            let next_deck1 = deck1
                .iter()
                .take(card1 as usize)
                .copied()
                .collect::<std::collections::VecDeque<u8>>();
            let next_deck2 = deck2
                .iter()
                .take(card2 as usize)
                .copied()
                .collect::<std::collections::VecDeque<u8>>();
            result = play_game(&next_deck1, &next_deck2, 2);
        } else {
            result = if card1 > card2 { 1 } else { 2 };
        }

        if result == 1 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    println!("Game 1 ran {} rounds", round);
    println!("{:?}", deck2);

    let deck1_value2 = deck1
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, card)| acc + (i + 1) * *card as usize);
    let deck2_value2 = deck2
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, card)| acc + (i + 1) * *card as usize);

    println!("Deck 1 value {}", deck1_value2);
    println!("Deck 2 value {}", deck2_value2);
}

fn play_game(
    deck1: &std::collections::VecDeque<u8>,
    deck2: &std::collections::VecDeque<u8>,
    game: u8,
) -> u8 {
    let mut configs1 = std::collections::HashSet::new();
    let mut configs2 = std::collections::HashSet::new();

    let mut new_deck1 = deck1.clone();
    let mut new_deck2 = deck2.clone();

    let mut round = 0;

    loop {
        round += 1;
        // println!("Game {}, round {}", game, round);
        // println!("{:?}", new_deck1);
        // println!("{:?}", new_deck2);
        if new_deck1.is_empty() {
            // println!("Deck 1 empty");
            return 2;
        }
        if new_deck2.is_empty() {
            // println!("Deck 2 empty");
            return 1;
        }

        if !configs1.insert(new_deck1.clone()) || !configs2.insert(new_deck2.clone()) {
            // println!("Repeated decks");
            return 1;
        }

        let card1 = new_deck1.pop_front().unwrap();
        let card2 = new_deck2.pop_front().unwrap();

        let result;

        if new_deck1.len() >= card1 as usize && new_deck2.len() >= card2 as usize {
            let next_deck1 = new_deck1
                .iter()
                .take(card1 as usize)
                .copied()
                .collect::<std::collections::VecDeque<u8>>();
            let next_deck2 = new_deck2
                .iter()
                .take(card2 as usize)
                .copied()
                .collect::<std::collections::VecDeque<u8>>();
            result = play_game(&next_deck1, &next_deck2, game + 1);
        } else {
            result = if card1 > card2 { 1 } else { 2 };
        }

        if result == 1 {
            new_deck1.push_back(card1);
            new_deck1.push_back(card2);
        } else {
            new_deck2.push_back(card2);
            new_deck2.push_back(card1);
        }
    }
}
