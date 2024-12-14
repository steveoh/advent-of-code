use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../day-10.data");
    let (map, trailheads) = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|c| c as usize)
                .enumerate()
                .map(move |(x, height)| ((x, y), height))
        })
        .fold(
            (HashMap::new(), Vec::new()),
            |(mut map, mut trailhead), (point, height)| {
                map.insert(point, height);
                if height == 0 {
                    trailhead.push(point);
                }
                (map, trailhead)
            },
        );

    let now = std::time::Instant::now();
    let part_one = trailheads
        .iter()
        .map(|&trailhead| depth_first_search(&map, trailhead))
        .sum::<usize>();
    println!("p1: {} ({:?})", part_one, now.elapsed());
    let now = std::time::Instant::now();
    let part_two = trailheads
        .iter()
        .map(|&trailhead| depth_first_visited(&map, trailhead))
        .sum::<usize>();
    println!("p2: {} ({:?})", part_two, now.elapsed());
}

const DIRECTIONS: [(isize, isize); 4] = [
    (0, 1),  // →
    (0, -1), // ←
    (1, 0),  // ↓
    (-1, 0), // ↑
];

fn get_next_step(
    map: &HashMap<(usize, usize), usize>,
    location: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut next = Vec::new();
    for d in DIRECTIONS.iter() {
        let trail = (location.0 as isize + d.0, location.1 as isize + d.1);
        if let Some(_) = map.get(&(trail.0 as usize, trail.1 as usize)).cloned() {
            next.push((trail.0 as usize, trail.1 as usize));
        }
    }
    next
}

fn depth_first_search(map: &HashMap<(usize, usize), usize>, location: (usize, usize)) -> usize {
    let mut nines = HashSet::new();
    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    stack.push((location, 0));

    while let Some((location, height)) = stack.pop() {
        if let Some(9) = map.get(&location) {
            nines.insert(location);

            continue;
        }

        for next_step in get_next_step(&map, location) {
            if visited.contains(&next_step) {
                continue;
            } else if map.get(&next_step) != Some(&(height + 1)) {
                continue;
            }

            visited.insert(next_step);
            stack.push((next_step, height + 1));
        }
    }

    return nines.len();
}

fn depth_first_visited(map: &HashMap<(usize, usize), usize>, location: (usize, usize)) -> usize {
    let mut stack = Vec::new();
    let mut rating = 0;

    stack.push((location, 0));

    while let Some((location, height)) = stack.pop() {
        if let Some(9) = map.get(&location) {
            rating += 1;
            continue;
        }

        for next_step in get_next_step(&map, location) {
            if map.get(&next_step) != Some(&(height + 1)) {
                continue;
            }

            stack.push((next_step, height + 1));
        }
    }

    return rating;
}
