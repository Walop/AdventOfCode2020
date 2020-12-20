fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let mut in_header = true;
    let mut tile_number = 0;
    let mut tiles = std::collections::HashMap::new();

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

            tiles.insert(tile_number, Vec::new());

            in_header = false;
        } else if line.is_empty() {
            in_header = true;
        } else {
            let mut tile_row = line.chars().collect::<Vec<_>>();
            tiles
                .entry(tile_number)
                .and_modify(|t| t.append(&mut tile_row));
        }
    }

    for (tile_number, tile_content) in &tiles {
        print!("\n{}", tile_number);
        for (i, c) in tile_content.iter().enumerate() {
            if i % side_len == 0 {
                print!("\n");
            }
            print!("{}", c);
        }
        print!("\n");
    }

    let mut side_mapping = std::collections::HashMap::new();

    for (tile_number, tile_content) in &tiles {
        let sides = get_sides(&tile_content, side_len);
        for side in sides {
            let mut rev_side = side.clone();
            let side_id = side_to_int(side);
            rev_side.reverse();
            let rev_side_id = side_to_int(rev_side);

            side_mapping
                .entry(side_id)
                .or_insert(std::collections::HashSet::new())
                .insert(tile_number);
            side_mapping
                .entry(rev_side_id)
                .or_insert(std::collections::HashSet::new())
                .insert(tile_number);
        }
    }

    let mut matching_sides = std::collections::HashMap::new();

    for (_, tile_numbers) in &side_mapping {
        if tile_numbers.len() > 1 {
            for tile_number in tile_numbers {
                matching_sides
                    .entry(tile_number)
                    .and_modify(|m| *m += 1)
                    .or_insert(1);
            }
        }
    }

    println!("{:?}", matching_sides);

    let product = matching_sides
        .iter()
        .filter(|(_, count)| **count == 4)
        .fold(1, |prod, (tile_number, _)| prod * ***tile_number as u64);

    println!("Product of corner tile numbers {}", product);

    let upper_left_number = **matching_sides
        .iter()
        .find(|(_, count)| **count == 4)
        .unwrap()
        .0;

    let mut upper_left = tiles.get(&upper_left_number).unwrap().clone();

    loop {
        let mut sides = get_sides(&upper_left, side_len);
        let mut right = side_to_int(sides[1].to_vec());
        let mut bottom = side_to_int(sides[2].to_vec());
        if side_mapping.get(&right).unwrap().len() > 1
            && side_mapping.get(&bottom).unwrap().len() > 1
        {
            break;
        }
        upper_left = mirror(&upper_left, side_len);
        sides = get_sides(&upper_left, side_len);
        right = side_to_int(sides[1].to_vec());
        bottom = side_to_int(sides[2].to_vec());
        if side_mapping.get(&right).unwrap().len() > 1
            && side_mapping.get(&bottom).unwrap().len() > 1
        {
            break;
        }
        upper_left = mirror(&upper_left, side_len);
        upper_left = rotate_right(&upper_left, side_len);
        // println!();
        // for (i, c) in upper_left.iter().enumerate() {
        //     if i % side_len == 0 {
        //         print!("\n");
        //     }
        //     print!("{}", c);
        // }
    }

    println!("Upper left number {}", upper_left_number);
    for (i, c) in upper_left.iter().enumerate() {
        if i % side_len == 0 {
            print!("\n");
        }
        print!("{}", c);
    }
    println!();

    let mut locked_tiles = vec![(upper_left_number, upper_left)];

    let image_width_in_tiles = (tiles.len() as f32).sqrt() as usize;

    while locked_tiles.len() < tiles.len() {
        let locked_len = locked_tiles.len();
        let mut compare_tile = locked_tiles.last().unwrap();
        let mut cmp_side = side_to_int(get_sides(&compare_tile.1, side_len)[1].to_vec());
        if locked_tiles.len() >= image_width_in_tiles {
            compare_tile = &locked_tiles[locked_tiles.len() - image_width_in_tiles];
            cmp_side = side_to_int(get_sides(&compare_tile.1, side_len)[2].to_vec());
        }
        let locked_tile_nums = locked_tiles.iter().map(|t| t.0).collect::<Vec<_>>();
        //println!("Locked tiles {:?}", locked_tile_nums);
        for (tile_number, tile_content) in &tiles {
            if locked_tile_nums.contains(&tile_number) {
                continue;
            }
            let mut found = false;
            //println!("Checking tile {}", tile_number);
            let mut sides = get_sides(tile_content, side_len);
            for (i, side) in sides.iter_mut().enumerate() {
                //println!("{:?}", side);
                let side_id = side_to_int(side.to_vec());
                //println!("{} cmp {}", cmp_side, side_id);
                if cmp_side == side_id {
                    if i == 0 {
                        let mut matching_tile = tile_content.to_vec();
                        if locked_tiles.len() < image_width_in_tiles {
                            matching_tile = mirror(&matching_tile, side_len);
                            matching_tile = rotate_right(&matching_tile, side_len);
                            matching_tile = rotate_right(&matching_tile, side_len);
                            matching_tile = rotate_right(&matching_tile, side_len);
                        }

                        locked_tiles.push((tile_number, matching_tile));
                        found = true;
                        break;
                    } else if i == 1 {
                        let mut matching_tile = tile_content.to_vec();
                        if locked_tiles.len() < image_width_in_tiles {
                            matching_tile = mirror(&matching_tile, side_len);
                            matching_tile = rotate_right(&matching_tile, side_len);
                            matching_tile = rotate_right(&matching_tile, side_len);
                        } else {
                            matching_tile = rotate_right(&matching_tile, side_len);
                            matching_tile = rotate_right(&matching_tile, side_len);
                            matching_tile = rotate_right(&matching_tile, side_len);
                        }

                        locked_tiles.push((tile_number, matching_tile));
                        found = true;
                        break;
                    } else if i == 2 {
                        let mut matching_tile = tile_content.to_vec();
                        if locked_tiles.len() < image_width_in_tiles {
                            matching_tile = rotate_right(&matching_tile, side_len);
                        } else {
                            matching_tile = mirror(&matching_tile, side_len);
                            matching_tile = rotate_right(&matching_tile, side_len);
                            matching_tile = rotate_right(&matching_tile, side_len);
                        }

                        locked_tiles.push((tile_number, matching_tile));
                        found = true;
                        break;
                    } else {
                        let mut matching_tile = tile_content.to_vec();

                        if locked_tiles.len() >= image_width_in_tiles {
                            matching_tile = mirror(&matching_tile, side_len);
                            matching_tile = rotate_right(&matching_tile, side_len);
                        }

                        locked_tiles.push((tile_number, matching_tile));
                        found = true;
                        break;
                    }
                }
                side.reverse();
                //println!("{:?}", side);
                let rev_side_id = side_to_int(side.to_vec());
                //println!("{} cmp {}", cmp_side, rev_side_id);
                if cmp_side == rev_side_id {
                    if i == 0 {
                        let mut matching_tile = tile_content.to_vec();
                        if locked_tiles.len() < image_width_in_tiles {
                            matching_tile = rotate_right(&matching_tile, side_len);
                            matching_tile = rotate_right(&matching_tile, side_len);
                            matching_tile = rotate_right(&matching_tile, side_len);
                        } else {
                            matching_tile = mirror(&matching_tile, side_len);
                        }

                        locked_tiles.push((tile_number, matching_tile));
                        found = true;
                        break;
                    } else if i == 1 {
                        let mut matching_tile = tile_content.to_vec();
                        if locked_tiles.len() < image_width_in_tiles {
                            matching_tile = rotate_right(&matching_tile, side_len);
                            matching_tile = rotate_right(&matching_tile, side_len);
                        } else {
                            matching_tile = mirror(&matching_tile, side_len);
                            matching_tile = rotate_right(&matching_tile, side_len);
                            matching_tile = rotate_right(&matching_tile, side_len);
                            matching_tile = rotate_right(&matching_tile, side_len);
                        }

                        locked_tiles.push((tile_number, matching_tile));
                        found = true;
                        break;
                    } else if i == 2 {
                        let mut matching_tile = tile_content.to_vec();
                        if locked_tiles.len() < image_width_in_tiles {
                            matching_tile = mirror(&matching_tile, side_len);
                            matching_tile = rotate_right(&matching_tile, side_len);
                        } else {
                            matching_tile = rotate_right(&matching_tile, side_len);
                            matching_tile = rotate_right(&matching_tile, side_len);
                        }

                        locked_tiles.push((tile_number, matching_tile));
                        found = true;
                        break;
                    } else {
                        let mut matching_tile = tile_content.to_vec();
                        if locked_tiles.len() < image_width_in_tiles {
                            matching_tile = mirror(&matching_tile, side_len);
                        } else {
                            matching_tile = rotate_right(&matching_tile, side_len);
                        }

                        locked_tiles.push((tile_number, matching_tile));
                        found = true;
                        break;
                    }
                }
            }
            if found {
                break;
            }
        }
        if locked_len == locked_tiles.len() {
            panic!("Nothing found");
        }
    }

    println!("{:?}", locked_tiles.iter().map(|t| t.0).collect::<Vec<_>>());
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

fn rotate_right(tile: &Vec<char>, tile_len: usize) -> Vec<char> {
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

    rotated
}

fn mirror(tile: &Vec<char>, tile_len: usize) -> Vec<char> {
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
    mirrored
}
