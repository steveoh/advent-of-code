use std::{cmp::Ordering, fs};

fn separate_rules_and_pages(file_path: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let parts = contents.split("\n\n").collect::<Vec<&str>>();
    // create a list of rule tuples
    let rules = parts[0]
        .lines()
        .map(|line| {
            let parts = line
                .split("|")
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            parts
        })
        .collect::<Vec<Vec<i32>>>();

    let updates = parts[1]
        .lines()
        .map(|line| {
            let parts = line
                .split(",")
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            parts
        })
        .collect::<Vec<Vec<i32>>>();

    (rules, updates)
}

fn is_page_valid(
    pages: Vec<i32>,
    less_than: Vec<i32>,
    greater_than: Vec<i32>,
    index: usize,
) -> bool {
    if !less_than
        .iter()
        .all(|&lt| index > pages.iter().position(|&x| x == lt).unwrap())
    {
        return false;
    }

    if !greater_than
        .iter()
        .all(|&gt| index < pages.iter().position(|&x| x == gt).unwrap())
    {
        return false;
    }

    true
}

fn part_one(updates: Vec<Vec<i32>>, rules: Vec<Vec<i32>>) {
    // 75,47,61,53,29
    let mut total = 0;
    for pages in updates.into_iter() {
        // find all rules that match the page (75)
        let mut valid = true;
        for (j, page) in pages.iter().enumerate() {
            // 75|29, 75|29, 75|47, 97|75, 75|61, 75|13
            let mut less_than = vec![];
            let mut greater_than = vec![];

            if !valid {
                break;
            }

            // populate less than and great than
            for rule in &rules {
                if !rule.contains(page) {
                    continue;
                }

                if *page != rule[0] && pages.contains(&rule[0]) {
                    less_than.push(rule[0]);
                } else if *page != rule[1] && pages.contains(&rule[1]) {
                    greater_than.push(rule[1]);
                }
            }

            if !is_page_valid(pages.clone(), less_than, greater_than, j) {
                valid = false;
                break;
            }
        }

        if valid {
            // println!("Pages: {:?} -> valid", pages);
            total += pages[pages.len() / 2];
        }
    }

    println!("Total: {:?}", total);
}

fn part_two(updates: Vec<Vec<i32>>, rules: Vec<Vec<i32>>) {
    let mut total = 0;
    let mut invalids = vec![];

    for pages in updates.into_iter() {
        let mut valid = true;

        for (j, page) in pages.iter().enumerate() {
            let mut less_than = vec![];
            let mut greater_than = vec![];

            if !valid {
                invalids.push(pages);

                break;
            }

            for rule in &rules {
                if !rule.contains(page) {
                    continue;
                }

                if *page != rule[0] && pages.contains(&rule[0]) {
                    less_than.push(rule[0]);
                } else if *page != rule[1] && pages.contains(&rule[1]) {
                    greater_than.push(rule[1]);
                }
            }

            valid = is_page_valid(pages.clone(), less_than, greater_than, j);
        }
    }

    for mut invalid in invalids {
        invalid.sort_by(|a, b| {
            rules
                .iter()
                .find_map(|rule| {
                    let before = rule[0];
                    let after = rule[1];

                    if *a == before && *b == after {
                        Some(Ordering::Less)
                    } else if *a == after && *b == before {
                        Some(Ordering::Greater)
                    } else {
                        None
                    }
                })
                .unwrap_or(Ordering::Equal)
        });

        total += invalid[invalid.len() / 2];
    }

    println!("Total: {:?}", total);
}

fn main() {
    let (rules, updates) = separate_rules_and_pages("src/day-5.data");

    // part_one(updates.clone(), rules.clone());
    part_two(updates.clone(), rules.clone());
}
