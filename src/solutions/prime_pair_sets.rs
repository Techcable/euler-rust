use std::slice;

use arrayvec::ArrayVec;

const LIMIT: u32 = 10_000;

#[derive(Clone, Debug)]
pub struct PrimePairSet(ArrayVec<[u32; 8]>);
impl PrimePairSet {
    #[inline]
    pub fn add(&self, value: u32) -> Self {
        let mut set = self.clone();
        set.0.push(value);
        set
    }
    pub fn iter(&self) -> slice::Iter<u32> {
        self.0.iter()
    }
}
#[derive(Copy, Clone, Debug)]
pub struct PrimePair(u32, u32);
impl PrimePair {
    #[inline]
    pub fn into_set(self) -> PrimePairSet {
        let mut set = ArrayVec::new();
        set.push(self.0);
        set.push(self.1);
        PrimePairSet(set)
    }
}
pub struct PrimePairCache {
    cache: Vec<Option<Vec<u32>>>
}
impl PrimePairCache {
    pub fn with_capacity(capacity: usize) -> Self {
        let primes = ::utils::primes(capacity);
        let mut cache = vec![None; capacity];
        for prime for primes {
            
        }
    }
    #[inline]
    fn primes(&self) -> impl Iterator<Item=u32> {
        self.cache.iter().enumerate()
            .filter(|(index, value)| value.is_some())
            .map(|(index, _)| index as u32)
    }
    #[inline]
    fn is_prime(&self, value: u32) -> bool {
        self.cache[value as usize].is_some()
    }
    #[inline]
    fn is_prime_pair(&self, first: u32, second: u32) -> bool {
        self.cache[first as usize].map_or(false, |valid| valid.contains(second))
    }
    #[inline]
    fn prime_pairs(&self, first: u32) -> PrimePairs {
        PrimePairs { first, second: self.cache[first as usize]
            .map_or(&[], Vec::as_slice).iter() }
    }
}
struct PrimePairs<'a> {
    first: u32,
    second: slice::Iter<'a, u32>
}
impl<'a> Iterator for PrimePairs<'a> {
    type Item = PrimePair;

    #[inline]
    fn next(&mut self) -> Option<PrimePair> {
        self.second.next().map(|&second| PrimePair(self.first, second))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.second.size_hint()
    }
}
impl<'a> ExactSizeIterator for PrimePairs<'a> {}

struct PrimePairSetFinder {
    cache: PrimePairCache,
}
impl PrimePairSetFinder {
    fn new() -> PrimePairSetFinder {
        PrimePairSetFinder {
            cache: PrimePairCache::with_capacity(10_000)
        }
    }
    fn find_sets(&self, length: usize) -> Vec<PrimePairSet> {
        assert!(length >= 2);
        if length == 2 {
            // the base set, taken from the cache
            let mut prime_sets = Vec::new();
            for prime in self.cache.primes() {
                for pair in self.cache.prime_pairs(prime) {
                    prime_sets.push(pair.into_set());
                }
            }
            prime_sets
        } else {
            // generate this level from the level one lower
            let old_sets = self.find_sets(length - 1);
            let mut possible_sets = Vec::new();
            for set in old_sets {
                let mut possibilities =
                    Vec::with_capacity(set.0.len() * set.0.len());
                for &value in set.iter() {
                    for potential_pair in self.cache.prime_pairs(value) {
                        // TODO: I think we only need to check `set`
                        if !possibilities.contains(potential_pair) {
                            possibilities.push(potential_pair);
                        }
                    }
                }
                possible_sets.extend(possibilities);
            }
            possible_sets
        }
    }
}

fn minimum_prime_pair_sum(len: usize) -> u32 {
    PrimePairSetFinder::wi
}

#[cfg(test)]
fn test() {

}
