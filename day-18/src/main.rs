use std::collections::{BinaryHeap, HashMap, HashSet};

// 70x70 grid
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct Position {
    x: usize,
    y: usize,
}
impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Eq, PartialEq, Clone)]
struct Progress {
    position: Position,
    score: usize,
    path: Vec<Position>,
}

impl Ord for Progress {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Progress {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Progress {
    fn new(position: Position, score: usize, path: Vec<Position>) -> Self {
        Self {
            position,
            score,
            path,
        }
    }
}

fn neighbors<T>(position: &Position, filter: T) -> Vec<Position>
where
    T: Fn(&Position) -> bool,
{
    [(0, -1), (-1, 0), (1, 0), (0, 1)]
        .iter()
        .map(|(dx, dy)| {
            let new_position = Position::new(
                (position.x as isize + dx) as usize,
                (position.y as isize + dy) as usize,
            );
            new_position
        })
        .filter(filter)
        .collect()
}

fn dijkstra(
    corrupted: &HashSet<Position>,
    start: Position,
    end: Position,
) -> Option<Vec<Position>> {
    let progress = Progress::new(start, 0, vec![start]);
    let mut visited: HashMap<Position, usize> = HashMap::new();
    let mut directions = BinaryHeap::new();
    directions.push(progress);

    while let Some(Progress {
        position,
        path,
        score,
    }) = directions.pop()
    {
        if let Some(&previous_score) = visited.get(&position) {
            if score >= previous_score {
                continue;
            }
        } else {
            visited.insert(position, score);
        }

        if position == end {
            return Some(path);
        }

        for neighbor in neighbors(&position, |pos| {
            pos.x <= end.x
                && pos.y <= end.y
                && !corrupted.contains(pos)
                && !visited.contains_key(&pos)
        }) {
            let mut path = path.clone();
            path.push(neighbor);

            directions.push(Progress::new(neighbor, score + 1, path));
        }
    }
    None
}

fn main() {
    let bytes = 12;
    let width = 6;
    let input = include_str!("../day-18.22");
    let mut corrupted: HashSet<Position> = HashSet::new();
    // let mut grid = vec![vec!['.'; width + 1]; width + 1];
    let start = Position::new(0, 0);
    let end = Position::new(width, width);

    input.lines().take(bytes).for_each(|line| {
        if let Some((x, y)) = line.split_once(',') {
            corrupted.insert(Position::new(
                x.parse::<usize>().unwrap(),
                y.parse::<usize>().unwrap(),
            ));
        }
    });

    if let Some(path) = dijkstra(&corrupted, start, end) {
        println!("{}", path.len() - 1);
        for y in 0..width + 1 {
            for x in 0..width + 1 {
                if path.contains(&Position::new(x, y)) {
                    print!("0");
                } else if corrupted.contains(&Position::new(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }

        // binary search

        input.lines().skip(bytes).for_each(|line| {
            if let Some((x, y)) = line.split_once(',') {
                let x = x.parse::<usize>().unwrap();
                let y = y.parse::<usize>().unwrap();
                if path.contains(&Position::new(x, y)) {
                    println!("{} {}", x, y);
                }
            }
        });
    }
}
