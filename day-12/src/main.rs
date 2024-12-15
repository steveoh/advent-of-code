use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../day-12.data");

    let garden: HashMap<(usize, usize), char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, plot)| ((x, y), plot))
        })
        .collect();

    let mut regions: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let now = std::time::Instant::now();
    for plot in garden.keys() {
        if visited.contains(plot) {
            continue;
        }

        let mut region: Vec<(usize, usize)> = Vec::new();
        let plant = garden[plot];

        get_region_for(plant, *plot, &garden, &visited, &mut region);

        visited.extend(&region);
        regions.push(region);
    }

    let total_region: usize = regions
        .iter()
        .map(|points| calculate_region_and_size(points))
        .sum();

    println!("Total cost: {:?} Time {:?}", total_region, now.elapsed());
}

fn get_region_for(
    plant: char,
    plot: (usize, usize),
    garden: &HashMap<(usize, usize), char>,
    visited: &HashSet<(usize, usize)>,
    region: &mut Vec<(usize, usize)>,
) {
    region.push(plot);
    let neighbors = get_neighbors(plot, &garden);
    let mut been_here: Vec<(usize, usize)> = Vec::new();

    for neighbor in neighbors {
        if !been_here.contains(&neighbor)
            && (plant == garden[&neighbor] && !region.contains(&neighbor))
        {
            been_here.push(neighbor);
            get_region_for(plant, neighbor, garden, visited, region);
        }

    }
}

fn get_neighbors(
    point: (usize, usize),
    garden: &HashMap<(usize, usize), char>,
) -> Vec<(usize, usize)> {
    let (x, y) = (point.0 as isize, point.1 as isize);
    vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
        .into_iter()
        .map(|(x, y)| (x as usize, y as usize))
        .filter(|plot| garden.contains_key(plot))
        .collect()
}
fn calculate_region_and_size(points: &Vec<(usize, usize)>) -> usize {
    let size = points.len();

    // for each point, start with 4 points and subtract 1 for every neighbor it has
    let region = points.iter().fold(0, |area, plot| {
        let (x, y) = (plot.0 as isize, plot.1 as isize);
        let shared_edges = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .iter()
            .filter(|point| point.0 >= 0 && point.1 >= 0)
            .map(|point| (point.0 as usize, point.1 as usize))
            .filter(|point| points.contains(point))
            .count();

        area + (4 - shared_edges)
    });

    region * size
}
