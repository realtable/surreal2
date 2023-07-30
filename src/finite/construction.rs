use std::collections::HashMap;
use std::f64::EPSILON;
use std::hash::{BuildHasher, Hash, Hasher};
use std::sync::Mutex;

use super::SurrealFinite;

lazy_static! {
    static ref CACHE: Mutex<HashMap<u64, SurrealStructure>> = Mutex::new(HashMap::new()); // serialise each value as part of key w/ serde
    static ref LEQ_MEMO: Mutex<HashMap<(u64, u64), bool>> = Mutex::new(HashMap::new());
}

#[derive(Clone, Hash)]
pub struct SurrealStructure {
    pub left: Vec<SurrealFinite>,
    pub right: Vec<SurrealFinite>,
}

impl SurrealStructure {
    pub fn new(left: Vec<SurrealFinite>, right: Vec<SurrealFinite>) -> SurrealStructure {
        SurrealStructure { left, right }
    }
}

pub fn cache_insert(structure: SurrealStructure) -> u64 {
    let mut hasher = CACHE.lock().unwrap().hasher().build_hasher();
    structure.hash(&mut hasher); // use hashing from https://github.com/ElsevierSoftwareX/SOFTX_2018_184/blob/master/src/SurrealFinite.jl instead?
    let hash: u64 = hasher.finish();

    let mut cache = CACHE.lock().unwrap();
    cache.entry(hash).or_insert(structure);

    hash
}

pub fn cache_left(hash: u64) -> Vec<SurrealFinite> {
    CACHE.lock().unwrap().get(&hash).unwrap().left.clone()
}

pub fn cache_right(hash: u64) -> Vec<SurrealFinite> {
    CACHE.lock().unwrap().get(&hash).unwrap().right.clone()
}

pub fn leq(x: &SurrealFinite, y: &SurrealFinite) -> bool {
    {
        let cache = LEQ_MEMO.lock().unwrap();
        if cache.contains_key(&(x.hash, y.hash)) {
            return *cache.get(&(x.hash, y.hash)).unwrap();
        }
    }

    let mut result = true;

    for xl in x.iter_left() {
        if leq(y, &xl) {
            result = false;
            break;
        }
    }

    if result {
        for yr in y.iter_right() {
            if leq(&yr, x) {
                result = false;
                break;
            }
        }
    }

    LEQ_MEMO.lock().unwrap().insert((x.hash, y.hash), result);
    result
}

/// Converts a floating-point number into a surreal number with finite sets.
pub fn ftos(f: f64) -> SurrealFinite {
    // add lazy evaluation?
    let zero = SurrealFinite::zero();
    let one = SurrealFinite::new(vec![zero], vec![]).unwrap();
    let neg_one = SurrealFinite::new(vec![], vec![zero]).unwrap();

    let mut increment = if f > 0.0 { one } else { neg_one };
    let mut large_bound = zero;
    let mut small_bound = zero;

    while (f - large_bound.stof()).abs() > EPSILON {
        // i.e. the best approximation with a finite float
        large_bound = small_bound;
        while f.abs() > large_bound.stof().abs() {
            small_bound = large_bound;
            large_bound += increment;
        }

        if increment > zero {
            increment = SurrealFinite::new(vec![zero], vec![increment]).unwrap();
        } else {
            increment = SurrealFinite::new(vec![increment], vec![zero]).unwrap();
        }
    }

    large_bound
}
