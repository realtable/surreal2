use std::fmt;
use std::ops;
use std::rc::Rc;

mod iterators;

pub use self::iterators::SurrealElement;
use self::iterators::*;
use super::finite::{ftos, SurrealFinite};

/// A representation of surreal numbers with potentially infinite sets.
#[derive(Clone)]
pub struct SurrealInfinite {
    left: Rc<dyn SurrealIterator>,
    right: Rc<dyn SurrealIterator>,
    name: Option<String>,
    value: Option<SurrealFinite>,
}

impl SurrealInfinite {
    pub fn new(
        left: Rc<dyn Fn(Option<SurrealElement>, usize) -> Option<SurrealElement>>,
        left_first: Option<SurrealElement>,
        right: Rc<dyn Fn(Option<SurrealElement>, usize) -> Option<SurrealElement>>,
        right_first: Option<SurrealElement>,
        name: Option<String>,
    ) -> SurrealInfinite {
        SurrealInfinite {
            left: Rc::new(SurrealBasicSet::new(left, left_first)),
            right: Rc::new(SurrealBasicSet::new(right, right_first)),
            name,
            value: None,
        }
    }

    fn new_raw(left: Rc<dyn SurrealIterator>, right: Rc<dyn SurrealIterator>) -> SurrealInfinite {
        SurrealInfinite {
            left,
            right,
            name: None,
            value: None,
        }
    }

    pub fn from_finite(x: SurrealFinite) -> SurrealInfinite {
        let vec_left: Vec<SurrealFinite> = x.iter_left().collect();
        let left = move |_, idx: usize| -> Option<SurrealElement> {
            if idx < vec_left.len() {
                Some(SurrealElement::Finite(vec_left[idx]))
            } else {
                None
            }
        };

        let vec_right: Vec<SurrealFinite> = x.iter_right().collect();
        let right = move |_, idx: usize| -> Option<SurrealElement> {
            if idx < vec_right.len() {
                Some(SurrealElement::Finite(vec_right[idx]))
            } else {
                None
            }
        };

        SurrealInfinite {
            left: Rc::new(SurrealBasicSet::new(Rc::new(left), None)),
            right: Rc::new(SurrealBasicSet::new(Rc::new(right), None)),
            name: None,
            value: Some(x),
        }
    }

    pub fn omega() -> SurrealInfinite {
        let left =
            |_, idx: usize| -> Option<SurrealElement> { Some(ftos(idx as f64 + 1.0).to_element()) };

        let right = |_, _| -> Option<SurrealElement> { None }; // helper function for empty closure

        SurrealInfinite::new(
            Rc::new(left),
            None,
            Rc::new(right),
            None,
            Some(String::from("ω")),
        )
    }

    pub fn epsilon() -> SurrealInfinite {
        let left = |_, idx: usize| -> Option<SurrealElement> {
            if idx == 0 {
                Some(SurrealFinite::zero().to_element())
            } else {
                None
            }
        }; // helper function for once closure

        let right = |prev: Option<SurrealElement>, _| -> Option<SurrealElement> {
            Some(
                SurrealFinite::new(
                    vec![SurrealFinite::zero()],
                    vec![prev.unwrap().coerce_finite()],
                )
                .unwrap()
                .to_element(),
            )
        };

        SurrealInfinite::new(
            Rc::new(left),
            None,
            Rc::new(right),
            Some(SurrealFinite::one().to_element()),
            Some(String::from("ϵ")),
        )
    }

    pub fn to_finite(&self, precision: usize) -> Option<SurrealFinite> {
        let recurse = |x: &SurrealElement| -> Option<SurrealFinite> {
            match x {
                SurrealElement::Finite(s) => Some(*s),
                SurrealElement::Infinite(s) => s.to_finite(precision),
            }
        };

        let left_trunc: Vec<Option<SurrealFinite>> =
            self.left.take(precision).iter().map(recurse).collect();
        let right_trunc: Vec<Option<SurrealFinite>> =
            self.right.take(precision).iter().map(recurse).collect();

        if left_trunc.contains(&None) || right_trunc.contains(&None) {
            None // if pseudo was returned
        } else {
            SurrealFinite::new(
                left_trunc.into_iter().flatten().collect(),
                right_trunc.into_iter().flatten().collect(),
            )
        }
    }

    pub fn to_element(&self) -> SurrealElement {
        SurrealElement::Infinite(self.clone())
    }
}

impl ops::Add<SurrealInfinite> for SurrealInfinite {
    type Output = SurrealInfinite;

    fn add(self, other: SurrealInfinite) -> SurrealInfinite {
        SurrealInfinite::new_raw(
            SurrealZipSet::new_rc(vec![
                SurrealAddSet::new_rc(self.to_element(), other.left.clone()),
                SurrealAddSet::new_rc(other.to_element(), self.left.clone()),
            ]),
            SurrealZipSet::new_rc(vec![
                SurrealAddSet::new_rc(self.to_element(), other.right.clone()),
                SurrealAddSet::new_rc(other.to_element(), self.right.clone()),
            ]),
        )
    }
}

impl ops::Neg for SurrealInfinite {
    type Output = SurrealInfinite;

    fn neg(self) -> SurrealInfinite {
        SurrealInfinite::new_raw(
            SurrealNegSet::new_rc(self.right),
            SurrealNegSet::new_rc(self.left),
        )
    }
}

impl ops::Sub<SurrealInfinite> for SurrealInfinite {
    type Output = SurrealInfinite;

    fn sub(self, other: SurrealInfinite) -> SurrealInfinite {
        self + (-other)
    }
}

// todo Mul, Rem, *Assign
// add function testing if a is 'close to' b (cos we cant do Eq or Ord if infinites are allowed to be pseudo surreal)

impl fmt::Display for SurrealInfinite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            Some(v) => write!(f, "{}", v.stof()),
            None => write!(f, "< {}| {}>", self.left, self.right),
        }
    }
}
