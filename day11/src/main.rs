#[derive(Clone, Copy, Debug)]
struct Vec2 {
    x: i32,
    y: i32
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {x: self.x + other.x, y: self.y + other.y};
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

fn main() {
    let orig_seatmap = std::fs::read_to_string("input.txt")
    .unwrap()
    .lines()
    .map(|l| l.chars().map(|c| if c == 'L' { '#' } else { c }).collect::<Vec<_>>())
    .collect::<Vec<_>>();

    let mut seatmap = orig_seatmap.clone();
    let mut changed = true;

    while changed {
        changed = false;
        let mut new_seatmap = seatmap.clone();
        for (r, row) in seatmap.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                if col == &'.' {
                    continue;
                }
                let mut occupied = 0;
                if r > 0 && c > 0 && seatmap[r-1][c-1] == '#' {
                    occupied += 1;
                }
                if r > 0 && seatmap[r-1][c] == '#' {
                    occupied += 1;
                }
                if r > 0 && c < row.len() - 1 && seatmap[r-1][c+1] == '#' {
                    occupied += 1;
                }
                if c > 0 && seatmap[r][c-1] == '#' {
                    occupied += 1;
                }
                if c < row.len() - 1 && seatmap[r][c+1] == '#' {
                    occupied += 1;
                }
                if r < seatmap.len() - 1 && c > 0 && seatmap[r+1][c-1] == '#' {
                    occupied += 1;
                }
                if r < seatmap.len() - 1 && seatmap[r+1][c] == '#' {
                    occupied += 1;
                }
                if r < seatmap.len() - 1 && c < row.len() - 1 && seatmap[r+1][c+1] == '#' {
                    occupied += 1;
                }

                if col == &'L' && occupied == 0 {
                    changed = true;
                    new_seatmap[r][c] = '#';
                }
                if col == &'#' && occupied > 3 {
                    changed = true;
                    new_seatmap[r][c] = 'L';
                }
            }
        }

        seatmap = new_seatmap;
    }

    let sum = seatmap.iter().map(|r| r.iter().map(|c| if c == &'#' { 1 } else { 0 }).sum::<i32>()).sum::<i32>();

    println!("First rule stable occupied seats {}", sum);

    seatmap = orig_seatmap.clone();
    changed = true;

    while changed {
        changed = false;
        let mut new_seatmap = seatmap.clone();
        for (r, row) in seatmap.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                if col == &'.' {
                    continue;
                }
                let mut occupied = 0;
                let position = Vec2 {x: c as i32, y: r as i32};
                for x in -1..=1 {
                    for y in -1..=1 {
                        occupied += line_of_sight(&seatmap, position, Vec2 {x: x, y: y})
                    }
                }

                if col == &'L' && occupied == 0 {
                    changed = true;
                    new_seatmap[r][c] = '#';
                }
                if col == &'#' && occupied > 4 {
                    changed = true;
                    new_seatmap[r][c] = 'L';
                }
            }
        }

        seatmap = new_seatmap;
    }

    let sum2 = seatmap.iter().map(|r| r.iter().map(|c| if c == &'#' { 1 } else { 0 }).sum::<i32>()).sum::<i32>();

    println!("Second rule stable occupied seats {}", sum2);
}

fn line_of_sight(seatmap: &std::vec::Vec<std::vec::Vec<char>>, location: Vec2, dir: Vec2) -> i32 {
    if dir.x == 0 && dir.y == 0 {
        return 0;
    }
    
    let mut pos = location + dir;
    while pos.x >= 0 && pos.x < seatmap[0].len() as i32 && pos.y >= 0 && pos.y < seatmap.len() as i32 {
        if seatmap[pos.y as usize][pos.x as usize] == 'L' {
            return 0;
        }
        if seatmap[pos.y as usize][pos.x as usize] == '#' {
            return 1;
        }
        pos += dir;
    }
    0
}