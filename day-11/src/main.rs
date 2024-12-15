use std::collections::HashMap;

// rules
// 1. If 0, replaced by 1.
// 2. If even number, it is replaced. divide the digits by 2 and split them.
// 3. If none of the other rules apply, multiply by 2024
const BLINKS: usize = 75;

fn main() {
    let mut already_computed: HashMap<(usize, usize), usize> = HashMap::new();
    // const STONES: [usize; 2] = [125, 17];
    const STONES: [usize; 8] = [30, 71441, 3784, 580926, 2, 8122942, 0, 291];

    let now = std::time::Instant::now();
    let stones: usize = STONES
        .iter()
        .map(|stone_value| compute(*stone_value, 1, &mut already_computed))
        .sum();

    println!("Part One {} Time taken: {:?}", stones, now.elapsed());
}

fn compute(
    stone_value: usize,
    blinks: usize,
    already_computed: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if blinks > BLINKS {
        return 1;
    }

    if let Some(result) = already_computed.get(&(stone_value, blinks)) {
        return *result;
    }

    let digits: String = stone_value.to_string();

    if stone_value == 0 {
        let stone_count = compute(1, blinks + 1, already_computed);
        already_computed.insert((stone_value, blinks), stone_count);

        return stone_count;
    } else if digits.len() % 2 == 0 {
        let half = digits.len() / 2;

        let left = digits[half..].parse::<usize>().unwrap();
        let right = digits[..half].parse::<usize>().unwrap();

        let stone_count = compute(left, blinks + 1, already_computed)
            + compute(right, blinks + 1, already_computed);
        already_computed.insert((stone_value, blinks), stone_count);

        return stone_count;
    } else {
        let stone_count = compute(stone_value * 2024, blinks + 1, already_computed);
        already_computed.insert((stone_value, blinks), stone_count);

        return stone_count;
    };
}
