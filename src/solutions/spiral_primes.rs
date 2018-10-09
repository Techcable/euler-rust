use failure::Error;
use ndarray::{Array2, ArrayView2};
use num::rational::Ratio;

use euler::EulerContext;
use super::EulerProblem;

#[derive(Debug, Clone)]
struct NumberSpiral(Array2<u64>);
impl NumberSpiral {
    fn initial() -> NumberSpiral {
        NumberSpiral(Array2::from_elem((1, 1), 1))
    }
    fn with_size(size: usize) -> NumberSpiral {
        assert_ne!(size, 0, "Can't have empty spiral");
        Self::initial().expand_repeatedly(size - 1)
    }
    fn size(&self) -> usize {
        let (x, y) = self.0.dim();
        assert_eq!(x, y);
        x
    }
    fn last(&self) -> u64 {
        let size = self.size();
        self.0[(size - 1, size - 1)]
    }
    fn expand_repeatedly(&self, amount: usize) -> NumberSpiral {
        let mut result = self.clone();
        for _ in 0..amount {
            result = result.expand();
        }
        result
    }
    fn expand(&self) -> NumberSpiral {
        let old_size = self.size();
        let new_size = old_size + 2;
        let old_last = self.last();
        trace!("Old last: {}", old_last);
        NumberSpiral(Array2::from_shape_fn(
            (new_size, new_size),
            |(a, b)| {
                let y = new_size - a - 1;
                let x = b;
                let maybe_existing = if a != 0 && b != 0 {
                    self.0.get((a - 1, b - 1))
                } else {
                    None
                };
                if let Some(&existing) = maybe_existing {
                    trace!("Existing {} @ ({}, {})", existing, x, y);
                    existing
                } else {
                    /*
                     * Determine which of the outer row/columns we're in.
                     * When there's overlap, the rows should take precedence,
                     * This cleanly handles exiting from the previous spiral,
                     * since that will be considered the first element of the right column.
                     */
                    let row_size = new_size as u64;
                    let column_size = new_size as u64 - 1;
                    let part;
                    let v = match (x, y) {
                        // NOTE: Match on y first so rows take precedence
                        (_, 0) => {
                            part = "bottom row";
                            // bottom row (logically last)
                            old_last + column_size * 2 + row_size + x as u64 - 1
                        },
                        (_, y) if y == new_size - 1 => {
                            part = "top row";
                            // top row (logically second)
                            old_last + column_size + (row_size - x as u64 - 1)
                        },
                        (0, _) => {
                            part = "left column";
                            // left column (logically third)
                            old_last + column_size + row_size + (column_size - y as u64 - 1)
                        }
                        (x, _) if x == new_size - 1 => {
                            part = "right column";
                            // right column (logically first)
                            old_last + (y as u64)
                        },
                        (_, _) => {
                            unreachable!("location ({}, {}) for old size {}", x, y, old_size)
                        }
                    };
                    trace!("Computed {} for {} @ ({}, {})", v, part, x, y);
                    v
                }
            }
        ))
    }
    fn diagonal_positions(&self) -> Vec<(usize, usize)> {
        let size = self.size();
        let center = (size / 2, size / 2);
        let mut diagonals = vec![center];
        /*
         * NOTE: We must exclude the center or there will be overlap.
         * There are two diagonals, one running from the lower left and one from the top left.
         * To get the lower left we must start at (0, 0)
         * and keep incrementing till we reach the end.
         * To get the top left we mus start at (0, size - 1)
         * and keep on incrementing the x while decrementing the y.
         * Both diagonals ignore the center (included separately),
         * since otherwise there'd be overlap.
         */
        let mut lower_left = (0, 0);
        while self.0.get(lower_left).is_some() {
            if lower_left != center {
                diagonals.push(lower_left);
            }
            lower_left.0 += 1;
            lower_left.1 += 1;
        }
        assert_eq!(lower_left, (size, size));
        let mut top_left = (0, size - 1);
        while self.0.get(top_left).is_some() {
            if top_left != center {
                diagonals.push(top_left);
            }
            if top_left.1 == 0 { break } // guards against subtracting with overflow
            top_left.0 += 1;
            top_left.1 -= 1;
        }
        // NOTE: This is actually valid because of the break
        assert_eq!(top_left, (size -1 , 0));
        diagonals
    }
    fn diagonals<'a>(&'a self) -> impl Iterator<Item=u64> + 'a {
        self.diagonal_positions().into_iter().map(move |pos| self.0[pos])
    }
    #[inline]
    fn prime_ratio(&self) -> Ratio<u64> {
        let primes = ::utils::prime_set(self.last() + 1);
        let mut count = 0;
        let mut total = 0;
        for value in self.diagonals() {
            if primes.contains(value as usize) {
                count += 1;
            }
            total += 1;
        }
        Ratio::new(count, total)
    }
    #[inline]
    fn view(&self) -> ArrayView2<u64> {
        self.0.view()
    }
}

#[derive(Default)]
pub struct SpiralPrimeProblem;
impl EulerProblem for SpiralPrimeProblem {
    fn name(&self) -> &'static str {
        "spiral_primes"
    }

    fn solve(&self, _context: &EulerContext) -> Result<String, Error> {
        let mut spiral = NumberSpiral::with_size(2);
        let threshold = Ratio::new(1, 10);
        loop {
            let ratio = spiral.prime_ratio();
            info!(
                "Ratio {} for spiral size {}",
                *ratio.numer() as f64 / *ratio.denom() as f64,
                spiral.size()
            );
            if ratio < threshold { break }
            spiral = spiral.expand();
        }
        Ok(format!("{}", spiral.size()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ndarray::aview2;
    const SECOND_EXPECTED: &[[u64; 3]] = &[
        [5, 4, 3],
        [6, 1, 2],
        [7, 8, 9]
    ];
    const THIRD_EXPECTED: &[[u64; 5]] = &[
        [17, 16, 15, 14, 13],
        [18,  5,  4,  3, 12],
        [19,  6,  1,  2, 11],
        [20,  7,  8,  9, 10],
        [21, 22, 23, 24, 25]
    ];
    const FOURTH_EXPECTED: &[[u64; 7]] = &[
        [37, 36, 35, 34, 33, 32, 31],
        [38, 17, 16, 15, 14, 13, 30],
        [39, 18,  5,  4,  3, 12, 29],
        [40, 19,  6,  1,  2, 11, 28],
        [41, 20,  7,  8,  9, 10, 27],
        [42, 21, 22, 23, 24, 25, 26],
        [43, 44, 45, 46, 47, 48, 49]
    ];
    #[test]
    fn check_answer() {

    }
    #[test]
    fn given_prime_ratio() {
        // They told us it'd be 8/3 for a spiral level 4
        let spiral = NumberSpiral::with_size(4);
        assert_eq!(
            spiral.prime_ratio(),
            Ratio::new(8, 13)
        );
    }
    #[test]
    fn spiral() {
        let first = NumberSpiral::initial();
        assert_eq!(first.view(), aview2(&[[1]]));
        let second = first.expand();
        assert_eq!(second.view(), aview2(SECOND_EXPECTED));
        let third = second.expand();
        assert_eq!(third.view(), aview2(THIRD_EXPECTED));
        let fourth = third.expand();
        assert_eq!(fourth.view(), aview2(FOURTH_EXPECTED));
    }
    #[test]
    fn spiral_diagonals() {
        let spiral = NumberSpiral::with_size(4);
        assert_eq!(
            spiral.diagonal_positions(),
            &[
                (3, 3), // NOTE: Skip center from now on
                // from lower left
                (0, 0),
                (1, 1),
                (2, 2),
                (4, 4),
                (5, 5),
                (6, 6),
                // from upper left
                (0, 6),
                (1, 5),
                (2, 4),
                (4, 2),
                (5, 1),
                (6, 0)
            ]
        )
    }
}