use crate::solution::SolutionTuple;
use crate::util::measure::MeasureContext;

type PreparedInput = Vec<u32>;

fn prepare(input: &str) -> PreparedInput {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

const PRUNE: u32 = 16777216;

/// Work with an internal representation (with less pruning) to speed up the inner loop.
/// To convert the internal state to the number, internal_to_number should be used.
fn evolve_internal(mut number: u32) -> u32 {
    number ^= number << 6;
    number ^= (number % PRUNE) >> 5;
    number ^= number << 11;

    number
}
fn internal_to_number(number: u32) -> u32 {
    number % PRUNE
}

fn evolve_iter(mut number: u32) -> impl Iterator<Item = u32> {
    std::iter::from_fn(move || {
        number = evolve_internal(number);
        Some(number)
    })
}

const SECRET_NUMBERS: usize = 2000;

fn price(number: u32) -> u32 {
    number % 10
}

fn solve_both(input: &PreparedInput) -> (u64, u32) {
    let mut p1 = 0;

    // Within the map, also keep track of the list item it was inserted with. This is used for the only-first check.
    // Implemented using an array. At 8 bytes * 19^4, this uses about 1 MB of memory
    let mut map = vec![(u32::MAX, 0u32); 19 * 19 * 19 * 19];

    input
        .iter()
        .enumerate()
        .for_each(|(list_item_index, number)| {
            let list_item_index = list_item_index as u32;

            let mut last_number = 0;
            let mut prices = evolve_iter(*number)
                .take(SECRET_NUMBERS)
                .map(internal_to_number)
                .inspect(|num| last_number = *num)
                .map(price);

            let mut previous_price = prices.next().unwrap();
            let mut window = 0u32;
            let mut price_window_iter = prices.map(|price| {
                // Price will go from -9 to 9. Map difference from 0..=18
                // This is 5 bytes, shift window by 8 bytes so that its values are shifted out of scope after 4 shifts (32 bytes)
                let price_diff = price + 9 - previous_price;
                previous_price = price;

                window <<= 8;
                window |= price_diff;

                (price, window)
            });

            // Window will not be valid for the first three iterations
            for _ in 0..3 {
                price_window_iter.next().unwrap();
            }

            for (price, window) in price_window_iter {
                // Turn the window using most of the u32 range into something smaller so a dumber map (an array) can be used
                let window_idx = (window >> 24) * 19 * 19 * 19
                    + ((window >> 16) % (1 << 8)) * 19 * 19
                    + ((window >> 8) % (1 << 8)) * 19
                    + window % (1 << 8);

                let entry = &mut map[window_idx as usize];
                // Only add if the current entry was from a previous item
                if entry.0 != list_item_index {
                    *entry = (list_item_index, entry.1 + price);
                }
            }

            p1 += last_number as u64;
        });

    (p1, map.into_iter().map(|(_, sum)| sum).max().unwrap())
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let input = ctx.measure("prepare", || prepare(input));
    ctx.measure("both", || solve_both(&input)).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PART1: &str = "1
10
100
2024";
    #[test]
    fn prepare_example() {
        assert_eq!(prepare(EXAMPLE_PART1).len(), 4);
    }
    #[test]
    fn evolve_123() {
        assert_eq!(
            evolve_iter(123)
                .take(10)
                .map(internal_to_number)
                .collect::<Vec<_>>(),
            [
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254,
            ]
        );
    }
    #[test]
    fn part1_example() {
        assert_eq!(solve_both(&prepare(EXAMPLE_PART1)).0, 37327623);
    }
    #[test]
    fn part2_example() {
        assert_eq!(
            solve_both(&prepare(
                "1
2
3
2024"
            ))
            .1,
            23
        );
    }
}
