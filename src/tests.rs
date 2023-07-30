use super::ftos;
use super::SurrealFinite;
use super::SurrealInfinite;

fn day_gen(days: i32) -> Vec<SurrealFinite> {
    if days == 1 {
        return vec![SurrealFinite::zero()];
    }

    let v = day_gen(days - 1);
    let mut w = vec![];

    w.push(SurrealFinite::new(vec![], vec![v[0]]).unwrap());
    for i in 0..v.len() {
        w.push(v[i]);
        if i != v.len() - 1 {
            w.push(SurrealFinite::new(vec![v[i]], vec![v[i + 1]]).unwrap());
        }
    }
    w.push(SurrealFinite::new(vec![v[v.len() - 1]], vec![]).unwrap());

    w
}

#[test]
fn leq_theorems() {
    let v = day_gen(6); // chosen for efficiency

    for i in 0..v.len() {
        assert!(v[i] <= v[i]); // T3

        for j in 0..v.len() {
            if !(v[i] <= v[j]) {
                assert!(v[j] <= v[i]); // T4
            }

            for k in 0..v.len() {
                if v[i] <= v[j] && v[j] <= v[k] {
                    assert!(v[i] <= v[k]); // T1

                    if &v[i] < &v[j] || &v[j] < &v[k] {
                        assert!(&v[i] < &v[k]); // T5 & T6
                    }
                }
            }
        }
    }
}

#[test]
fn add_theorems() {
    let v = day_gen(4);

    for i in 0..v.len() {
        assert!(v[i] + SurrealFinite::zero() == v[i]); // T10

        for j in 0..v.len() {
            assert!(v[i] + v[j] == v[j] + v[i]); // T9

            for k in 0..v.len() {
                assert!((v[i] + v[j]) + v[k] == v[i] + (v[j] + v[k])); // T11

                if v[i] == v[j] {
                    assert!(v[i] + v[k] == v[j] + v[k]) // T12
                }
            }
        }
    }
}

#[test]
fn neg_theorems() {
    let v = day_gen(4);

    for i in 0..v.len() {
        assert!(v[i] - v[i] == SurrealFinite::zero()); // T15
        assert!(-(-v[i]) == v[i]); // T16

        for j in 0..v.len() {
            assert!((v[i] + v[j]) - v[j] == v[i]); // T17
            assert!(-(v[i] + v[j]) == (-v[i]) + (-v[j]));
        }
    }
}

#[test]
fn mul_theorems() {
    let v = day_gen(3);

    for i in 0..v.len() {
        assert!(v[i] * SurrealFinite::zero() == SurrealFinite::zero()); // T21
        assert!(v[i] * SurrealFinite::one() == v[i]); // T22

        for j in 0..v.len() {
            assert!(v[i] * v[j] == v[j] * v[i]); // T20
            assert!(-(v[i] * v[j]) == (-v[i]) * v[j]); // T23
        }
    }
}

#[test]
fn stof_ftos() {
    let v = day_gen(7);

    for i in 0..v.len() {
        assert!(v[i] == ftos(v[i].stof()));
    }
}

#[test]
fn omega() {
    println!("ω = {}", SurrealInfinite::omega());
    println!("ϵ = {}", SurrealInfinite::epsilon());
    println!("-ω = {}", -SurrealInfinite::omega());
    println!(
        "2 - 1 = {}",
        ftos(2.0).to_infinite() - SurrealFinite::one().to_infinite()
    );
    // println!(
    //     "ω - 1 = {}",
    //     SurrealInfinite::omega() - SurrealFinite::one().to_infinite()
    // );
    // println!(
    //     "ϵ + 1 = {}",
    //     SurrealInfinite::epsilon() + SurrealFinite::one().to_infinite()
    // );
    // println!(
    //     "ω + ω = {}",
    //     SurrealInfinite::omega() + SurrealInfinite::omega()
    // );
    // println!(
    //     "ϵ * ω = {}",
    //     SurrealInfinite::epsilon() * SurrealInfinite::omega()
    // );
}

// todo: rem, assign, infinite, fmt
