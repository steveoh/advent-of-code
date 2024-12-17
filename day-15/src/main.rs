use std::vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Item {
    Wall,
    Box,
    Empty,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}
struct Position {
    position: (usize, usize),
    type_: Item,
}
impl Position {
    fn new(position: (usize, usize), type_: Item) -> Self {
        Self { position, type_ }
    }
}

fn print_map(map: &Vec<Vec<Position>>, robot: (usize, usize)) {
    let mut output = String::new();
    for (y, row) in map.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            if (x, y) == robot {
                output.push('@');
            } else {
                output.push(match item.type_ {
                    Item::Wall => '#',
                    Item::Box => 'O',
                    Item::Empty => '.',
                });
            }
        }
        println!("{}", output);
        output = String::new();
    }
}

fn gps((y, x): (usize, usize)) -> usize {
    (x * 100) + y
}

fn main() {
    let input = include_str!("../day-15.data");
    let (raw_map, raw_instructions) = input.split_once("\n\n").unwrap();
    let mut map = vec![];

    let now = std::time::Instant::now();

    let mut robot = (0, 0);
    for (y, line) in raw_map.lines().enumerate() {
        map.push(vec![]);

        for (x, c) in line.chars().enumerate() {
            let item = match c {
                '#' => Some(Position::new((x, y), Item::Wall)),
                '.' => Some(Position::new((x, y), Item::Empty)),
                'O' => Some(Position::new((x, y), Item::Box)),
                '@' => {
                    robot = (x, y);
                    Some(Position::new((x, y), Item::Empty))
                }
                _ => None,
            };
            if let Some(item) = item {
                map[y].push(item);
            }
        }
    }

    let instructions: Vec<((isize, isize), Instruction)> = raw_instructions
        .lines()
        .filter_map(|line| Some(line.trim()))
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '<' => ((-1, 0), Instruction::Left),
                '>' => ((1, 0), Instruction::Right),
                '^' => ((0, -1), Instruction::Up),
                'v' => ((0, 1), Instruction::Down),
                _ => unreachable!(),
            })
        })
        .collect();

    println!("{:?}", instructions.len());

    for (direction, _) in instructions {
        // println!("moved {:?}", instruction);

        let (x, y) = robot;
        let (dx, dy) = direction;
        let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);

        let item = map[ny][nx].type_;
        if item == Item::Wall {
            continue;
        } else if item == Item::Empty {
            robot = (nx, ny);

            continue;
        } else if item == Item::Box {
            // in the direction of travel is there an empty space?
            // find it and shift the boxes and robot
            let (mut bx, mut by) = ((nx as isize + dx) as usize, (ny as isize + dy) as usize);
            while &map[by][bx].type_ == &Item::Box {
                bx = (bx as isize + dx) as usize;
                by = (by as isize + dy) as usize;
            }

            if &map[by][bx].type_ == &Item::Empty {
                map[ny][nx].type_ = Item::Empty;
                map[by][bx].type_ = Item::Box;
                robot = (nx, ny);
            } else {
                continue;
            }
        }
    }

    print_map(&map, robot);
    // ##########
    // #.O.O.OOO#
    // #........#
    // #OO......#
    // #OO@.....#
    // #O#.....O#
    // #O.....OO#
    // #O.....OO#
    // #OO....OO#
    // ##########
    // calc distance from 0,0
    let result: usize = map
        .iter()
        .flatten()
        .filter(|pos| pos.type_ == Item::Box)
        .map(|pos| gps(pos.position))
        .sum();

    println!("Result: {:?} Elapsed: {:?}", result, now.elapsed());
}
