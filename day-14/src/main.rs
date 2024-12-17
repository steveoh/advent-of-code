use regex::Regex;

#[derive(Debug)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
    bathroom: (isize, isize),
}
impl Robot {
    fn new(position: (isize, isize), velocity: (isize, isize), bathroom: (isize, isize)) -> Self {
        Self {
            position,
            velocity,
            bathroom,
        }
    }

    fn tick(&mut self, seconds: isize) {
        self.position.0 += self.velocity.0 * seconds;
        self.position.1 += self.velocity.1 * seconds;

        if self.position.0 < 0 || self.position.0 > self.bathroom.0 {
            self.position.0 = self.position.0.rem_euclid(self.bathroom.0 + 1);
        }

        if self.position.1 < 0 || self.position.1 > self.bathroom.1 {
            self.position.1 = self.position.1.rem_euclid(self.bathroom.1 + 1);
        }
    }

    fn place(&self) -> Option<usize> {
        let x = self.position.0;
        let y = self.position.1;

        let max_x = self.bathroom.0;
        let max_y = self.bathroom.1;

        if x < max_x / 2 && y < max_y / 2 {
            return Some(0);
        } else if x > max_x / 2 && y < max_y / 2 {
            return Some(1);
        } else if x < max_x / 2 && y > max_y / 2 {
            return Some(2);
        } else if x > max_x / 2 && y > max_y / 2 {
            return Some(3);
        }

        return None;
    }
}

fn part_two() {
    // iterate all robots until they are in the same space
    // calculate the lower common multiple of the seconds
    // for loop from 0 to lcm and get the first empty space
    // flood fill the space to get a count of empty spaces
    // hope the tree is in the iteration with the least empty spaces
}
fn main() {
    // p=0,4 v=3,-3
    let robot_regex: Regex =
        Regex::new(r"p=(?P<x>-?\d+),(?P<y>-?\d+)\sv=(?P<x2>-?\d+),(?P<y2>-?\d+)").unwrap();
    let input = include_str!("../day-14.data");
    let space = (101, 103);
    let seconds = 100;

    let now = std::time::Instant::now();
    let robots: usize = input
        .lines()
        .filter_map(|line| {
            robot_regex.captures(line).map(|caps| {
                let position = (
                    caps["x"].parse::<isize>().unwrap(),
                    caps["y"].parse::<isize>().unwrap(),
                );
                let velocity = (
                    caps["x2"].parse::<isize>().unwrap(),
                    caps["y2"].parse::<isize>().unwrap(),
                );
                Robot::new(position, velocity, (space.0 - 1, space.1 - 1))
            })
        })
        .map(|mut robot| {
            robot.tick(seconds);

            robot.place()
        })
        .filter_map(|x| x)
        .fold(vec![0; 4], |mut acc, x| {
            acc[x] += 1;
            acc
        })
        .iter()
        .product();

    println!("Tokens: {:?} Elapsed: {:?}", robots, now.elapsed());
}
