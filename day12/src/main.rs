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

impl std::ops::Mul<i32> for Vec2 {
    type Output = Self;
    
    fn mul(self, other: i32) -> Self {
        Self {x: self.x * other, y: self.y * other}
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    instruction: char,
    value: i32
}

fn main() {
    let instructions = std::fs::read_to_string("input.txt")
    .unwrap()
    .lines()
    .map(|l| Instruction { instruction: l.chars().next().unwrap(), value: l.chars().skip(1).collect::<String>().parse::<i32>().unwrap() })
    .collect::<Vec<_>>();

    let dirs = [
        Vec2 { x: 1, y: 0 },
        Vec2 { x: 0, y: -1 },
        Vec2 { x: -1, y: 0 },
        Vec2 { x: 0, y: 1 }
    ];
    let mut current_dir = 0;

    let mut pos = Vec2 {x: 0, y: 0};

    for i in &instructions {
        match i.instruction {
            'E' => pos += dirs[0] * i.value,
            'S' => pos += dirs[1] * i.value,
            'W' => pos += dirs[2] * i.value,
            'N' => pos += dirs[3] * i.value,
            'F' => pos += dirs[current_dir] * i.value,
            'R' => current_dir = (current_dir + (i.value / 90) as usize) % 4,
            'L' => current_dir = (current_dir + 4 - (i.value / 90) as usize) % 4,
            _ => panic!()
        }
    }

    println!("Part 1");
    println!("End position {:?}", pos);
    println!("Manhattan distance from origin {}", pos.x.abs() + pos.y.abs());
    println!();

    let mut waypoint = Vec2 { x: 10, y: 1 };
    pos = Vec2 {x: 0, y: 0};

    for i in &instructions {
        match i.instruction {
            'E' => waypoint += dirs[0] * i.value,
            'S' => waypoint += dirs[1] * i.value,
            'W' => waypoint += dirs[2] * i.value,
            'N' => waypoint += dirs[3] * i.value,
            'F' => pos += waypoint * i.value,
            _ => waypoint = rotate(&waypoint, i.instruction, i.value / 90)
        }
    }

    println!("Part 2");
    println!("End position {:?}", pos);
    println!("Manhattan distance from origin {}", pos.x.abs() + pos.y.abs());
}

fn rotate(waypoint: &Vec2, dir: char, times: i32) -> Vec2 {
    let mut new_waypoint =  waypoint.clone();
    for _ in 0..times {
        if dir == 'L' {
            new_waypoint = Vec2 { x: -new_waypoint.y, y: new_waypoint.x}
        }
        else {
            new_waypoint = Vec2 { x: new_waypoint.y, y: -new_waypoint.x}
        }
    }
    new_waypoint
}