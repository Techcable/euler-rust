//! An implementation of sieve of Eratosthenes
use fixedbitset::FixedBitSet;

use super::DebugTimer;

/// Make a bitset of all primes less than the specified value.
///
/// Internally this uses the sieve of Eratosthenes for simplicity,
/// as it's very fast for finding prime values.
pub fn prime_set(limit: u64) -> FixedBitSet {
    assert!(limit <= (usize::max_value() as u64));
    let timer = DebugTimer::start();
    let mut is_prime = FixedBitSet::with_capacity(limit as usize);
    is_prime.set_range(2.., true);
    for i in 2..((limit as f64).sqrt().ceil() as usize) {
        if is_prime[i] {
            let mut j = i * i;
            while j < (limit as usize) {
                is_prime.set(j, false);
                j += i;
            }
        }
    }
    timer.finish_with(|| format!("Computed prime set of {}", limit));
    is_prime
}

/// List of all primes less than the specified value.
///
/// Internally this is just a simple wrapper around `prime_set`.
pub fn primes(limit: u64) -> Vec<u64> {
    prime_set(limit).ones().map(|i| i as u64).collect()
}

const BFSZ: u64 = 1 << 16;
const BFBTS: u64 = BFSZ * 32;
const BFRNG: u64 = BFBTS * 2;

/// An incremental sieve of Eratosthenes
///
/// This uses a very fast page segmentation algorithm,
/// translated from the [Java version on rosetta code](https://web.archive.org/web/20181009211844/https://rosettacode.org/wiki/Sieve_of_Eratosthenes#Infinite_iterator_with_a_very_fast_page_segmentation_algorithm_.28sieves_odds-only.29).
pub struct IncrementalSieve {
    bi: Option<u64>,
    lowi: u64,
    bpa: Vec<u32>,
    bps: Option<Box<IncrementalSieve>>,
    // TODO: Should this be inline?
    buf: Box<[u32; BFSZ as usize]>
}
impl IncrementalSieve {
    pub fn new() -> Self {
        IncrementalSieve {
            bi: None,
            lowi: 0,
            bpa: Vec::new(),
            bps: None,
            buf: box [0u32; BFSZ as usize],
        }
    }
    pub fn next_prime(&mut self) -> u64 {
        match self.bi {
            None => {
                self.bi = Some(0);
                return 2
            }
            Some(0) => {
                let nxt = 3 + (self.lowi << 1) + BFRNG;
                if self.lowi <= 0 { // special culling for first page as no base primes yet:
                    let mut i = 0;
                    let mut p = 3;
                    let mut sqr = 9;
                    while sqr < nxt {
                        if (self.buf[(i >> 5) as usize] & (1 << (i & 31))) == 0 {
                            let mut j = (sqr - 3) >> 1;
                            while j < BFBTS {
                                self.buf[(j >> 5) as usize] |= 1 << (j & 31);
                                j += p;
                            }
                        }
                        i += 1;
                        p += 2;
                        sqr = p * p;
                    }
                } else { // after the first page
                    // clear the sieve buffer
                    ::utils::clear_slice(&mut *self.buf);
                    // initialize separate base primes stream:
                    let bps = &mut **self.bps
                        .get_or_insert_with(|| Box::new(Self::new()));
                    if self.bpa.is_empty() { // if this is the first page after the zero one:
                        // advance past the only even prime of two
                        debug_assert_eq!(bps.next_prime(), 2);
                        // get the next prime (3 in this case)
                        self.bpa.push(bps.next_prime() as u32);
                        debug_assert_eq!(self.bpa.last(), Some(&3));
                    }
                    {
                        // get enough base primes for the page range...
                        let mut p = *self.bpa.last().unwrap() as u64;
                        let mut sqr = p * p;
                        while sqr < nxt {
                            p = bps.next_prime();
                            self.bpa.push(p as u32);
                            sqr = p * p;
                        }
                    }
                    for &p in &self.bpa[..(self.bpa.len() - 1)] {
                        let p = p as u64;
                        let mut s = (p * p - 3) >> 1;
                        if s >= self.lowi {
                            // adjust start index based on page lower limit...
                            s -= self.lowi;
                        } else {
                            let r = (self.lowi - s) % p;
                            s = if r != 0 { p - r } else { 0 };
                        }
                        {
                            let mut j = s as u32;
                            while j < BFBTS as u32 {
                                self.buf[(j >> 5) as usize] |= 1 << (j & 31);
                                j += p as u32;
                            }
                        }
                    }
                }
            },
            Some(_) => {}, // we have primes remaining in the buffer
        }
        {
            // find next marker still with prime status
            // NOTE: We temporarily update bi in to a local variable for convenience
            let mut bi = self.bi.unwrap();
            while (bi < BFBTS)
                && ((self.buf[(bi >> 5) as usize] & (1 << (bi as u32 & 31))) != 0) {
                bi += 1;
            }
            self.bi = Some(bi);
        }
        let bi = self.bi.unwrap();
        if bi < BFBTS {
            // within buffer: output computed prime
            let prime = 3 + ((self.lowi + bi) << 1);
            self.bi = Some(bi + 1);
            prime
        } else {
            // beyond buffer range: advance buffer
            self.bi = Some(0);
            self.lowi += BFBTS;
            // and recursively loop
            self.next_prime()
        }
    }
}
impl Iterator for IncrementalSieve {
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<u64> {
        Some(self.next_prime())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_incremental() {
        ::env_logger::init();
        let n = 1_000_000;
        let timer = DebugTimer::start();
        let incremental_primes = IncrementalSieve::new()
            .take_while(|&i| i < n)
            .collect::<Vec<u64>>();
        timer.finish_with(|| format!("Incrementally computed {} primes", n));
        let primes = primes(n);
        assert_eq!(incremental_primes, primes);
    }
}
