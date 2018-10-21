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



/// Tests if a value is prime
///
/// Internally uses the Miller–Rabin primality test
pub fn is_prime(value: u64) -> bool {
    fn witness(n: u64, mut s: u64, d: u64, a: u32) -> bool {
        // NOTE: Ported from RosettaCode C version
        let mut x = ::utils::modular_pow(a as u64, d, n);
        let mut y = 0;
        debug_assert_ne!(s, 0);
        while s != 0 {
            y = (x * x) % n;
            if y == 1 && x != 1 && x != n-1 {
                return false;
            }
            x = y;
            s -= 1;
        }
        y == 1
    }
    if value < 2 {
        return false;
    } else if value == 2 {
        return true;
    } else if value % 2 == 0 {
        // Even numbers greater than two are composite
        return false;
    }
    let mut d = value / 2;
    let mut s = 1;
    while d & 1 == 0 {
        d /= 2;
        s += 1;
    }
    needed_witnesses(value).iter()
        .all(|&a| witness(value, s, d, a))
}

/// The witnessess needed to make the miller-rabbin test deterministic
fn needed_witnesses(value: u64) -> &'static [u32] {
    /*
     * These 'witnesses' are taken from wikipedia page
     * and make the tests deterministic assuming the value is below a certian limit.
     * I've optimized this if chain for readability, not for speed
     */
    if value < 2047 {
        &[2]
    } else if value < 1_373_653 {
        &[2, 3]
    } else if value < 9_080_191 {
        &[31, 73]
    } else if value < 25_326_001 {
        &[2, 3, 5]
    } else if value < 3_215_031_751 {
        &[2, 3, 5, 7]
    } else if value < 4_759_123_141 {
        &[2, 7, 61]
    } else if value < 2_152_302_898_747 {
        &[2, 3, 5, 7, 11]
    } else if value < 3_474_749_660_383 {
        &[2, 3, 5, 7, 11, 13]
    } else if value < 341_550_071_728_321 {
        &[2, 3, 5, 7, 11, 13, 17]
    } else {
        // Since value is always less than 2**64, this will always work
        &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
    }
}

const BFSZ: u64 = 1 << 16;
const BFBTS: u64 = BFSZ * 32;
const BFRNG: u64 = BFBTS * 2;

/// A set of primes that expands incrementally
#[deprecated(note = "Gave bad answers on Problem 58")]
pub struct IncrementalPrimeSet {
    set: FixedBitSet,
    #[allow(deprecated)]
    sieve: IncrementalSieve
}
#[allow(deprecated)]
impl IncrementalPrimeSet {
    pub fn new() -> IncrementalPrimeSet {
        IncrementalPrimeSet {
            set: FixedBitSet::default(),
            sieve: IncrementalSieve::new()
        }
    }
    pub fn with_initial_limit(limit: u64) -> IncrementalPrimeSet {
        let mut set = IncrementalPrimeSet::new();
        set.expand(limit);
        set
    }
    pub fn expand(&mut self, limit: u64) {
        if limit <= self.limit() { return }
        let old_limit = self.limit();
        assert!(limit <= (usize::max_value() as u64));
        let timer = DebugTimer::start();
        let primes = self.sieve.by_ref()
            .take_while(|&p| p < limit);
        self.set.grow(limit as usize);
        let mut count = 0;
        let mut start = None;
        for prime in primes {
            if start.is_none() { start = Some(prime); }
            self.set.insert(prime as usize);
            count += 1;
        }
        timer.finish_with(|| {
            use std::fmt::Display;
            let start = start.as_ref().map_or_else(
                || &"N/A" as &Display,
                |i| i as &Display
            );
            format!(
                "Expanded prime set from {} to {}: found {} primes starting with {}",
                old_limit, limit, count, start
            )
        })
    }
    #[inline]
    pub fn limit(&self) -> u64 {
        self.set.len() as u64
    }
    #[inline]
    pub fn contains(&self, prime: u64) -> bool{
        assert!(prime <= (self.set.len() as u64));
        self.set.contains(prime as usize)
    }
    /// Check if the specified value is prime,
    /// lazily expanding the set if needed
    pub fn check_prime(&mut self, value: u64) -> bool {
        if value >= self.limit() {
            let old_limit = self.limit();
            self.expand((value + 1000).max(old_limit * 2));
        }
        self.contains(value)
    }
}

/// An incremental sieve of Eratosthenes
///
/// This uses a very fast page segmentation algorithm,
/// translated from the [Java version on rosetta code](https://web.archive.org/web/20181009211844/https://rosettacode.org/wiki/Sieve_of_Eratosthenes#Infinite_iterator_with_a_very_fast_page_segmentation_algorithm_.28sieves_odds-only.29).
#[deprecated(note = "Gave bad answers on Problem 58")]
pub struct IncrementalSieve {
    bi: Option<u64>,
    lowi: u64,
    bpa: Vec<u32>,
    bps: Option<Box<IncrementalSieve>>,
    // TODO: Should this be inline?
    buf: Box<[u32; BFSZ as usize]>
}
#[allow(deprecated)]
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
    pub fn generate_primes_until(&mut self, limit: u64) -> Vec<u64> {
        let timer = DebugTimer::start();
        let primes = self.take_while(|&n| n < limit)
            .collect::<Vec<_>>();
        timer.finish_with(|| {
            use std::fmt::Display;
            let start = primes.last().map_or_else(
                || &"N/A" as &Display,
                |i| i as &Display
            );
            format!("Incrementally computed {} primes until {}, starting with {}", primes.len(), limit, start)
        });
        primes
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
#[allow(deprecated)]
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
    #[allow(deprecated)]
    fn test_incremental() {
        ::env_logger::init();
        let n = 1_000_000;
        let primes = primes(n);
        let incremental_primes = IncrementalSieve::new().
            generate_primes_until(n);
        assert_eq!(incremental_primes, primes);
    }
}
