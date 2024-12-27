use crate::solution::SolutionTuple;
use crate::util::measure::MeasureContext;
use std::simd::{LaneCount, Simd, SupportedLaneCount};

type PreparedInput = Vec<u32>;

fn prepare(input: &str) -> PreparedInput {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

const PRUNE: u32 = 16777216;

/// Work with an internal representation (with less pruning) to speed up the inner loop.
/// To convert the internal state to the number, internal_to_number should be used.
fn evolve_internal<const N: usize>(mut number: Simd<u32, N>) -> Simd<u32, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    number ^= number << 6;
    number ^= (number % Simd::splat(PRUNE)) >> 5;
    number ^= number << 11;

    number
}
fn internal_to_number<const N: usize>(number: Simd<u32, N>) -> Simd<u32, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    number % Simd::splat(PRUNE)
}
fn evolve_iter<const N: usize>(mut number: Simd<u32, N>) -> impl Iterator<Item = Simd<u32, N>>
where
    LaneCount<N>: SupportedLaneCount,
{
    std::iter::from_fn(move || {
        number = evolve_internal(number);
        Some(number)
    })
}

const SECRET_NUMBERS: usize = 2000;

fn price<const N: usize>(number: Simd<u32, N>) -> Simd<u32, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    number % Simd::splat(10)
}

const LANE: usize = 8;

fn solve_both(input: &PreparedInput) -> (u64, u32) {
    let mut p1 = 0;

    // Within the map, also keep track of the list item it was inserted with. This is used for the only-first check.
    // Implemented using an array. At 8 bytes * 19^4, this uses about 1 MB of memory
    let mut map = vec![0u32; 19 * 19 * 19 * 19];
    let nineteen_simd: Simd<u32, LANE> = Simd::splat(19);
    let nineteen_simd_2 = nineteen_simd * nineteen_simd;
    let nineteen_simd_3 = nineteen_simd_2 * nineteen_simd;
    let eight_byte_mask: Simd<u32, LANE> = Simd::splat((1 << 8) - 1);

    let mut found = vec![0u8; 19 * 19 * 19 * 19];

    input.chunks(LANE).for_each(|chunk| {
        found.iter_mut().for_each(|elem| *elem = 0);

        let numbers: Simd<u32, LANE> = Simd::load_or_default(chunk);

        let mut last_number = Simd::splat(0);

        let mut prices = evolve_iter(numbers)
            .take(SECRET_NUMBERS)
            .map(internal_to_number)
            .inspect(|num| last_number = *num)
            .map(price);

        let mut previous_price = prices.next().unwrap();
        let mut window = Simd::splat(0u32);
        let mut price_window_iter = prices.map(|price| {
            // Price will go from -9 to 9. Map difference from 0..=18
            // This is 5 bytes, shift window by 8 bytes so that its values are shifted out of scope after 4 shifts (32 bytes)
            let price_diff = price + Simd::splat(9) - previous_price;
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
            let window_idx = (window >> 24) * nineteen_simd_3
                + ((window >> 16) & eight_byte_mask) * nineteen_simd_2
                + ((window >> 8) & eight_byte_mask) * nineteen_simd
                + (window & eight_byte_mask);

            for i in 0..chunk.len() {
                let window_idx = window_idx[i] as usize;

                // Only add if the current entry was from a previous item
                let found = &mut found[window_idx];
                if (*found >> i) & 1 == 0 {
                    let entry = &mut map[window_idx];
                    *entry += price[i];
                    *found |= 1 << i;
                }
            }
        }

        p1 += last_number
            .as_array()
            .iter()
            .map(|last_number| *last_number as u64)
            .sum::<u64>();
    });

    (p1, map.into_iter().max().unwrap())
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
            evolve_iter(Simd::from_array([123]))
                .take(10)
                .map(internal_to_number)
                .map(|simd| simd[0])
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
