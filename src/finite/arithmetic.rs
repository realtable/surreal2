use std::collections::HashMap;
use std::sync::Mutex;

use super::{ftos, SurrealFinite};

// memoise these calculations?

lazy_static! {
    static ref ADD_MEMO: Mutex<HashMap<(u64, u64), u64>> = Mutex::new(HashMap::new());
    static ref NEG_MEMO: Mutex<HashMap<u64, u64>> = Mutex::new(HashMap::new());
    static ref MUL_MEMO: Mutex<HashMap<(u64, u64), u64>> = Mutex::new(HashMap::new());
}

pub fn add(x: SurrealFinite, y: SurrealFinite) -> SurrealFinite {
    {
        let cache = ADD_MEMO.lock().unwrap();
        if cache.contains_key(&(x.hash, y.hash)) {
            return SurrealFinite {
                hash: *cache.get(&(x.hash, y.hash)).unwrap(),
            };
        }
    }

    let mut new_left: Vec<SurrealFinite> = Vec::new();
    for xl in x.iter_left() {
        new_left.push(add(xl, y));
    }
    for yl in y.iter_left() {
        new_left.push(add(yl, x));
    }

    let mut new_right: Vec<SurrealFinite> = Vec::new();
    for xr in x.iter_right() {
        new_right.push(add(xr, y));
    }
    for yr in y.iter_right() {
        new_right.push(add(yr, x));
    }

    let result = SurrealFinite::new(new_left, new_right).unwrap(); // doesnt need to be handled if x and y are non-pseudo
    ADD_MEMO
        .lock()
        .unwrap()
        .insert((x.hash, y.hash), result.hash);
    result
}

pub fn neg(x: SurrealFinite) -> SurrealFinite {
    {
        let cache = NEG_MEMO.lock().unwrap();
        if cache.contains_key(&x.hash) {
            return SurrealFinite {
                hash: *cache.get(&x.hash).unwrap(),
            };
        }
    }

    let mut new_left: Vec<SurrealFinite> = Vec::new();
    for xr in x.iter_right() {
        new_left.push(neg(xr));
    }

    let mut new_right: Vec<SurrealFinite> = Vec::new();
    for xl in x.iter_left() {
        new_right.push(neg(xl));
    }

    let result = SurrealFinite::new(new_left, new_right).unwrap();
    NEG_MEMO.lock().unwrap().insert(x.hash, result.hash);
    result
}

pub fn mul(x: SurrealFinite, y: SurrealFinite) -> SurrealFinite {
    {
        let cache = MUL_MEMO.lock().unwrap();
        if cache.contains_key(&(x.hash, y.hash)) {
            return SurrealFinite {
                hash: *cache.get(&(x.hash, y.hash)).unwrap(),
            };
        }
    }

    let mut new_left: Vec<SurrealFinite> = Vec::new();
    for xl in x.iter_left() {
        for yl in y.iter_left() {
            new_left.push(add(add(mul(xl, y), mul(x, yl)), neg(mul(xl, yl))));
        }
    }
    for xr in x.iter_right() {
        for yr in y.iter_right() {
            new_left.push(add(add(mul(xr, y), mul(x, yr)), neg(mul(xr, yr))));
        }
    }

    let mut new_right: Vec<SurrealFinite> = Vec::new();
    for xl in x.iter_left() {
        for yr in y.iter_right() {
            new_right.push(add(add(mul(xl, y), mul(x, yr)), neg(mul(xl, yr))));
        }
    }
    for xr in x.iter_right() {
        for yl in y.iter_left() {
            new_right.push(add(add(mul(xr, y), mul(x, yl)), neg(mul(xr, yl))));
        }
    }

    let result = SurrealFinite::new(new_left, new_right).unwrap();
    MUL_MEMO
        .lock()
        .unwrap()
        .insert((x.hash, y.hash), result.hash);
    result
}

pub fn div_approx(x: SurrealFinite, y: SurrealFinite) -> SurrealFinite {
    ftos(x.stof() / y.stof())
}
