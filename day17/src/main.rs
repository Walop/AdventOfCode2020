#[derive(Copy,Clone,Debug, PartialEq, Eq, Hash)]
struct Point4 {
    x: i32,
    y: i32,
    z: i32,
    w: i32
}

fn main() {
    let mut initial_on_nodes = std::collections::HashSet::new();

    for line in std::fs::read_to_string("input.txt").unwrap().lines().enumerate() {
        for c in line.1.chars().enumerate() {
            if c.1 == '#' {
                initial_on_nodes.insert(Point4 {x: c.0 as i32, y: line.0 as i32, z: 0, w: 0});
            }
        }
    }

    let mut on_nodes = initial_on_nodes.clone();
    let cycles = match std::env::args().nth(1) {
        Some(a) => a.parse::<u32>().expect("Invalid cycle count"),
        None => 6
    };

    for cycle in 0..cycles {
        let mut next_on_nodes = on_nodes.clone();
        let mut with_neighbours_on = std::collections::HashMap::new();
        for node in &on_nodes {
            let (on, neighbour_on) = check_neighbours3(&on_nodes, &node);
            if !on {
                next_on_nodes.remove(node);
            }
            for neighbour in neighbour_on {
                with_neighbours_on.entry(neighbour).and_modify(|count| *count += 1).or_insert(1);
            }
        }
        for (node, count) in with_neighbours_on {
            if count == 3 {
                next_on_nodes.insert(node);
            }
        }

        on_nodes = next_on_nodes;
        println!("On nodes after {} cycles 3D {:?}", cycle+1, on_nodes.len());
    }

    println!("Xmax {}, Xmin {}", on_nodes.iter().max_by_key(|n| n.x).unwrap().x, on_nodes.iter().min_by_key(|n| n.x).unwrap().x);
    println!("Ymax {}, Ymin {}", on_nodes.iter().max_by_key(|n| n.y).unwrap().y, on_nodes.iter().min_by_key(|n| n.y).unwrap().y);
    println!("Zmax {}, Zmin {}", on_nodes.iter().max_by_key(|n| n.z).unwrap().z, on_nodes.iter().min_by_key(|n| n.z).unwrap().z);
    println!("Finished {} cycles 3D, nodes on: {:?}", cycles, on_nodes.len());

    on_nodes = initial_on_nodes.clone();

    for cycle in 0..cycles {
        let mut next_on_nodes = on_nodes.clone();
        let mut with_neighbours_on = std::collections::HashMap::new();
        for node in &on_nodes {
            let (on, neighbour_on) = check_neighbours4(&on_nodes, &node);
            if !on {
                next_on_nodes.remove(node);
            }
            for neighbour in neighbour_on {
                with_neighbours_on.entry(neighbour).and_modify(|count| *count += 1).or_insert(1);
            }
        }
        for (node, count) in with_neighbours_on {
            if count == 3 {
                next_on_nodes.insert(node);
            }
        }

        on_nodes = next_on_nodes;
        println!("On nodes after {} cycles 3D {:?}", cycle+1, on_nodes.len());
    }

    println!("Xmax {}, Xmin {}", on_nodes.iter().max_by_key(|n| n.x).unwrap().x, on_nodes.iter().min_by_key(|n| n.x).unwrap().x);
    println!("Ymax {}, Ymin {}", on_nodes.iter().max_by_key(|n| n.y).unwrap().y, on_nodes.iter().min_by_key(|n| n.y).unwrap().y);
    println!("Zmax {}, Zmin {}", on_nodes.iter().max_by_key(|n| n.z).unwrap().z, on_nodes.iter().min_by_key(|n| n.z).unwrap().z);
    println!("Wmax {}, Wmin {}", on_nodes.iter().max_by_key(|n| n.w).unwrap().w, on_nodes.iter().min_by_key(|n| n.w).unwrap().w);

    println!("Finished {} cycles 4D, nodes on: {:?}", cycles, on_nodes.len());
}

fn check_neighbours3(on_nodes: &std::collections::HashSet<Point4>, node: &Point4) -> (bool, Vec<Point4>) {
    let mut neighbours_on = 0;
    let mut off_neighbours = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                let point = Point4 {x: node.x+dx, y: node.y+dy, z: node.z+dz, w: 0};
                if node != &point {
                    if on_nodes.contains(&point) {
                        neighbours_on += 1;
                    }
                    else {
                        off_neighbours.push(point);
                    }
                }
            }
        }
    }

    (neighbours_on == 2 || neighbours_on == 3, off_neighbours)
}

fn check_neighbours4(on_nodes: &std::collections::HashSet<Point4>, node: &Point4) -> (bool, Vec<Point4>) {
    let mut neighbours_on = 0;
    let mut off_neighbours = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                for dw in -1..=1 {
                    let point = Point4 {x: node.x+dx, y: node.y+dy, z: node.z+dz, w: node.w+dw};
                    if node != &point {
                        if on_nodes.contains(&point) {
                            neighbours_on += 1;
                        }
                        else {
                            off_neighbours.push(point);
                        }
                    }
                }
            }
        }
    }

    (neighbours_on == 2 || neighbours_on == 3, off_neighbours)
}