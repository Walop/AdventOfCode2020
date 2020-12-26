#[derive(Clone, Debug)]
struct Tile {
    id: u16,
    data: Vec<char>,
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let mut in_header = true;
    let mut tile_number = 0;
    let mut tiles = Vec::new();

    let side_len = content.lines().next().unwrap().len();

    for line in content.lines() {
        if in_header {
            tile_number = line
                .chars()
                .skip_while(|c| !c.is_ascii_digit())
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u16>()
                .unwrap();

            tiles.push(Tile {
                id: tile_number,
                data: Vec::new(),
            });

            in_header = false;
        } else if line.is_empty() {
            in_header = true;
        } else {
            let mut tile_row = line.chars().collect::<Vec<_>>();
            let tile = tiles.iter_mut().find(|t| t.id == tile_number).unwrap();
            tile.data.append(&mut tile_row);
        }
    }

    //print_tiles(&tiles, side_len);

    let mut current = tiles.first().unwrap().clone();

    loop {
        let mut found = false;
        let compare_side = side_to_int(get_sides(&current.data, side_len)[0].to_vec());
        for tile in tiles.iter_mut() {
            if tile.id == current.id {
                continue;
            }
            for _ in 0..4 {
                let mut side = side_to_int(get_sides(&tile.data, side_len)[2].to_vec());
                if side == compare_side {
                    found = true;
                    current = tile.clone();
                    break;
                }
                mirror(&mut tile.data, side_len);
                side = side_to_int(get_sides(&tile.data, side_len)[2].to_vec());
                if side == compare_side {
                    found = true;
                    current = tile.clone();
                    break;
                }
                mirror(&mut tile.data, side_len);
                rotate_right(&mut tile.data, side_len);
            }
            if found {
                break;
            }
        }

        if !found {
            break;
        }
    }

    loop {
        let mut found = false;
        let compare_side = side_to_int(get_sides(&current.data, side_len)[3].to_vec());
        for tile in tiles.iter_mut() {
            if tile.id == current.id {
                continue;
            }
            for _ in 0..4 {
                let mut side = side_to_int(get_sides(&tile.data, side_len)[1].to_vec());
                if side == compare_side {
                    found = true;
                    current = tile.clone();
                    break;
                }
                mirror(&mut tile.data, side_len);
                side = side_to_int(get_sides(&tile.data, side_len)[1].to_vec());
                if side == compare_side {
                    found = true;
                    current = tile.clone();
                    break;
                }
                mirror(&mut tile.data, side_len);
                rotate_right(&mut tile.data, side_len);
            }
            if found {
                break;
            }
        }

        if !found {
            break;
        }
    }

    println!("{}", current.id);

    let mut used = vec![current];

    let tiles_width = (tiles.len() as f32).sqrt() as usize;

    while used.len() < tiles.len() {
        let compare_side;
        let mut next_side = 3;
        if used.len() < tiles_width {
            current = used.last().unwrap().clone();
            compare_side = side_to_int(get_sides(&current.data, side_len)[1].to_vec());
        } else {
            current = used[used.len() - tiles_width].clone();
            compare_side = side_to_int(get_sides(&current.data, side_len)[2].to_vec());
            next_side = 0;
        }
        for tile in tiles.iter_mut() {
            if used.iter().any(|t| t.id == tile.id) {
                continue;
            }
            for _ in 0..4 {
                let mut side = side_to_int(get_sides(&tile.data, side_len)[next_side].to_vec());
                if side == compare_side {
                    used.push(tile.clone());
                    break;
                }
                mirror(&mut tile.data, side_len);
                side = side_to_int(get_sides(&tile.data, side_len)[next_side].to_vec());
                if side == compare_side {
                    used.push(tile.clone());
                    break;
                }
                mirror(&mut tile.data, side_len);
                rotate_right(&mut tile.data, side_len);
            }
        }
    }

    let mut product = used.first().unwrap().id as u64;
    product *= used[tiles_width - 1].id as u64;
    product *= used[(tiles_width - 1) * tiles_width].id as u64;
    product *= used.last().unwrap().id as u64;

    println!("1. result {}", product);

    let trimmed_data = used
        .iter()
        .map(|t| remove_edges(&t.data, side_len))
        .collect::<Vec<_>>();

    let trimmed_len = side_len - 2;

    let mut combined_data = Vec::new();

    for i in 0..tiles_width {
        for j in 0..trimmed_len {
            for k in 0..tiles_width {
                combined_data.append(
                    &mut trimmed_data[i * tiles_width + k]
                        .iter()
                        .skip(j * trimmed_len)
                        .take(trimmed_len)
                        .cloned()
                        .collect::<Vec<_>>(),
                );
            }
        }
    }

    let combined_len = tiles_width * trimmed_len;

    let target = "                  # #    ##    ##    ### #  #  #  #  #  #   "
        .chars()
        .collect::<Vec<_>>();

    for _ in 0..4 {
        let mut found = find(&mut combined_data, &target, combined_len);
        if found {
            break;
        }
        mirror(&mut combined_data, combined_len);
        found = find(&mut combined_data, &target, combined_len);
        if found {
            break;
        }
        mirror(&mut combined_data, combined_len);
        rotate_right(&mut combined_data, combined_len);
    }

    for (i, c) in combined_data.iter().enumerate() {
        if i % (tiles_width * (trimmed_len)) == 0 {
            print!("\n");
        }
        print!("{}", c);
    }

    println!();

    println!(
        "2. result {}",
        combined_data.iter().filter(|c| c == &&'#').count()
    );
}

fn print_tiles(tiles: &Vec<Tile>, side_len: usize) {
    for tile in tiles {
        print!("\n{}", tile.id);
        for (i, c) in tile.data.iter().enumerate() {
            if i % side_len == 0 {
                print!("\n");
            }
            print!("{}", c);
        }
        print!("\n");
    }
}

fn get_sides(tile_content: &Vec<char>, side_len: usize) -> Vec<Vec<char>> {
    let mut sides = Vec::new();
    let top = tile_content
        .iter()
        .take(side_len)
        .cloned()
        .collect::<Vec<_>>();
    sides.push(top);
    let right = tile_content
        .iter()
        .skip(side_len - 1)
        .step_by(side_len)
        .cloned()
        .collect::<Vec<_>>();
    sides.push(right);
    let bottom = tile_content
        .iter()
        .skip(side_len * (side_len - 1))
        .cloned()
        .collect::<Vec<_>>();
    sides.push(bottom);
    let left = tile_content
        .iter()
        .step_by(side_len)
        .cloned()
        .collect::<Vec<_>>();
    sides.push(left);

    sides
}

fn side_to_int(side: Vec<char>) -> u16 {
    side.iter().enumerate().fold(
        0,
        |acc, (i, c)| if c == &'#' { acc + (1 << i) } else { acc },
    )
}

fn rotate_right(tile: &mut Vec<char>, tile_len: usize) {
    let mut rotated = Vec::new();
    for i in 0..tile_len {
        let mut row = tile
            .iter()
            .skip(i)
            .step_by(tile_len)
            .rev()
            .cloned()
            .collect::<Vec<_>>();
        //println!("{:?}", row);
        rotated.append(&mut row);
    }

    *tile = rotated;
}

fn mirror(tile: &mut Vec<char>, tile_len: usize) {
    let mut mirrored = Vec::new();
    for i in 0..tile_len {
        let mut row = tile
            .iter()
            .skip(i * tile_len)
            .take(tile_len)
            .rev()
            .cloned()
            .collect::<Vec<_>>();
        mirrored.append(&mut row);
    }
    *tile = mirrored;
}

fn remove_edges(data: &Vec<char>, side_len: usize) -> Vec<char> {
    let mut result = Vec::new();

    result.append(
        &mut data
            .iter()
            .skip(side_len + 1)
            .take(side_len - 2)
            .cloned()
            .collect::<Vec<_>>(),
    );
    for i in 2..side_len {
        result.append(
            &mut data
                .iter()
                .skip(i * side_len + 1)
                .take(side_len - 2)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }

    result
}

fn find(map: &mut Vec<char>, target: &Vec<char>, combined_len: usize) -> bool {
    let mut found = false;
    let target_non_empty = target
        .iter()
        .enumerate()
        .filter(|(_, x)| x == &&'#')
        .collect::<Vec<_>>();
    for i in 0..combined_len - 3 {
        for j in 0..combined_len - 20 {
            let pos = i * combined_len + j;
            let sum = target_non_empty
                .iter()
                .map(|(k, c)| {
                    if &&map[pos + (k / 20 * combined_len) + (k % 20)] == c {
                        1
                    } else {
                        0
                    }
                })
                .sum::<usize>();
            if sum == target_non_empty.len() {
                found = true;
                for x in target_non_empty
                    .iter()
                    .map(|(k, _)| pos + (k / 20 * combined_len) + (k % 20))
                {
                    map[x] = 'O';
                }
            }
        }
    }
    if found {
        return true;
    }

    false
}
