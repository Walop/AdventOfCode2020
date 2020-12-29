#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl<'a, 'b> std::ops::Add<&'b Point> for &'a Point {
    type Output = Point;

    fn add(self, other: &'b Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let dirs = vec![
        Point { x: 1, y: 1 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: -1 },
        Point { x: -1, y: -1 },
        Point { x: -1, y: 0 },
        Point { x: 0, y: 1 },
    ];

    let mut flipped = std::collections::HashSet::new();

    for line in content.lines() {
        let mut pos = Point { x: 0, y: 0 };

        let mut iter = line.chars();
        while let Some(c) = iter.next() {
            match c {
                'n' => match iter.next().unwrap() {
                    'e' => pos += dirs[0],
                    'w' => pos += dirs[5],
                    _ => panic!("Unknown dir"),
                },
                's' => match iter.next().unwrap() {
                    'e' => pos += dirs[2],
                    'w' => pos += dirs[3],
                    _ => panic!("Unknown dir"),
                },
                'e' => pos += dirs[1],
                'w' => pos += dirs[4],
                _ => panic!("Unknown dir"),
            }
        }

        if !flipped.insert(pos) {
            flipped.remove(&pos);
        }
    }

    println!("Black tiles after first phase: {}", flipped.len());

    let iters = 100;

    for _ in 0..iters {
        let mut next_flipped = flipped.clone();
        let mut empty_n = std::collections::HashMap::new();

        for tile in &flipped {
            let mut n_count = 0;
            for dir in &dirs {
                let pos = tile + dir;
                if flipped.contains(&pos) {
                    n_count += 1;
                } else {
                    empty_n.entry(pos).and_modify(|c| *c += 1).or_insert(1);
                }
            }
            if n_count == 0 || n_count > 2 {
                next_flipped.remove(&tile);
            }
        }

        empty_n
            .into_iter()
            .filter(|(_, count)| count == &2)
            .for_each(|(pos, _)| {
                next_flipped.insert(pos);
            });
        flipped = next_flipped;
    }

    println!("Black tiles after {} iterations: {}", iters, flipped.len());
}
