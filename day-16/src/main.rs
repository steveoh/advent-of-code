use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Position {
    fn is_linear(&self, other: &Position, direction: &Direction) -> bool {
        match direction {
            Direction::North | Direction::South => self.x == other.x,
            Direction::East | Direction::West => self.y == other.y,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn neighbors(&self, position: &Position) -> Vec<(Position, Direction)> {
        match self {
            Direction::North => vec![
                (Position::new(position.x, position.y - 1), Direction::North),
                (Position::new(position.x + 1, position.y), Direction::East),
                (Position::new(position.x - 1, position.y), Direction::West),
            ],
            Direction::South => vec![
                (Position::new(position.x, position.y + 1), Direction::South),
                (Position::new(position.x + 1, position.y), Direction::East),
                (Position::new(position.x - 1, position.y), Direction::West),
            ],
            Direction::East => vec![
                (Position::new(position.x + 1, position.y), Direction::East),
                (Position::new(position.x, position.y + 1), Direction::South),
                (Position::new(position.x, position.y - 1), Direction::North),
            ],
            Direction::West => vec![
                (Position::new(position.x - 1, position.y), Direction::West),
                (Position::new(position.x, position.y + 1), Direction::South),
                (Position::new(position.x, position.y - 1), Direction::North),
            ],
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
struct Progress {
    position: Position,
    direction: Direction,
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
    fn new(position: Position, direction: Direction, score: usize, path: Vec<Position>) -> Self {
        Self {
            position,
            direction,
            score,
            path,
        }
    }

    fn calculate_score(&self, next: &Position) -> usize {
        if self.position.is_linear(next, &self.direction) {
            return 1 + self.score;
        } else {
            return 1001 + self.score;
        }
    }
}

fn create_maze(input: &str) -> (HashSet<Position>, Position, Position) {
    return input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .fold(
            (HashSet::new(), Position::new(0, 0), Position::new(0, 0)),
            |(mut maze, mut start, mut end), (x, y, c)| {
                if c == 'S' {
                    start = Position::new(x, y);
                } else if c == 'E' {
                    end = Position::new(x, y);
                }

                if c != '#' {
                    maze.insert(Position::new(x, y));
                }
                (maze, start, end)
            },
        );
}

fn dijkstra(maze: &HashSet<Position>, start: Position, end: Position) -> usize {
    let progress = Progress::new(start, Direction::East, 0, vec![start]);
    let mut visited: HashMap<(Position, Direction), usize> = HashMap::new();
    let mut directions = BinaryHeap::new();
    directions.push(progress);
    visited.insert((start, Direction::East), 0);

    while let Some(Progress {
        position,
        direction,
        path,
        score,
    }) = directions.pop()
    {
        if let Some(&previous_score) = visited.get(&(position, direction)) {
            if score > previous_score {
                continue;
            }
        } else {
            visited.insert((position, direction), score);
        }

        if position == end {
            return score;
        }

        for neighbor in direction
            .neighbors(&position)
            .iter()
            .filter(|(pos, _)| maze.contains(pos) && !visited.contains_key(&(*pos, direction)))
        {
            let mut path = path.clone();
            path.push(neighbor.0);

            let score = Progress::new(position, direction, score, path.clone())
                .calculate_score(&neighbor.0);
            directions.push(Progress::new(neighbor.0, neighbor.1, score, path));
        }
    }
    0
}

fn main() {
    let (maze, start, end) = create_maze(include_str!("../day-16.data"));

    let now = std::time::Instant::now();
    let score = dijkstra(&maze, start, end);
    println!("Score: {} Elapsed: {:?}", score, now.elapsed());
}
