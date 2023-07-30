use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops;

mod arithmetic;
mod construction;

pub use self::arithmetic::div_approx;
pub use self::construction::ftos;
use self::construction::{cache_insert, cache_left, cache_right, SurrealStructure};
use super::infinite::{SurrealElement, SurrealInfinite};

/// A representation of surreal numbers with finite sets.
#[derive(Clone, Copy, Debug)] // should debug be derived here?
pub struct SurrealFinite {
    hash: u64,
}

impl SurrealFinite {
    /// Creates a new surreal number given two finite vectors of surreal numbers. Each vector corresponds to a left set and a right set, where all numbers in the left set must be less than all numbers in the right set.
    ///
    /// Returns `None` if any number in the left set is greater than any number in the right set.
    ///
    /// Surreal numbers composed of non-finite sets are instead represented by [`SurrealInfinite`].
    ///
    /// # Examples
    ///
    /// ```
    /// let zero = surreal::SurrealFinite::new(vec![], vec![]).unwrap();
    /// let one = surreal::SurrealFinite::new(vec![zero], vec![]).unwrap();
    /// let neg_one = surreal::SurrealFinite::new(vec![], vec![zero]).unwrap();
    /// ```
    pub fn new(left: Vec<SurrealFinite>, right: Vec<SurrealFinite>) -> Option<SurrealFinite> {
        let x = SurrealFinite::new_unchecked(left, right);

        if !x.left_is_empty()
            && !x.right_is_empty()
            && x.iter_left().last() >= x.iter_right().next()
        {
            return None;
        }

        Some(x) // returns finite, non-pseudo surreal numbers
                // use Err instead of Option?
    }

    fn new_unchecked(mut left: Vec<SurrealFinite>, mut right: Vec<SurrealFinite>) -> SurrealFinite {
        left.sort();
        right.sort();
        let s_structure = SurrealStructure::new(left, right);

        let hash = cache_insert(s_structure);

        SurrealFinite { hash } // returns finite surreal numbers (potentially pseudo)
    }

    /// Returns the surreal number representing zero.
    ///
    /// # Examples
    ///
    /// ```
    /// let zero = surreal::SurrealFinite::zero();
    /// let zero_manual = surreal::SurrealFinite::new(vec![], vec![]).unwrap();
    /// assert!(zero == zero_manual);
    /// ```
    pub fn zero() -> SurrealFinite {
        SurrealFinite::new_unchecked(vec![], vec![])
    }

    /// Returns the surreal number representing one.
    ///
    /// # Examples
    ///
    /// ```
    /// let one = surreal::SurrealFinite::one();
    /// let one_manual = surreal::SurrealFinite::new(vec![surreal::SurrealFinite::zero()], vec![]).unwrap();
    /// assert!(one == one_manual);
    /// ```
    pub fn one() -> SurrealFinite {
        SurrealFinite::new_unchecked(vec![SurrealFinite::zero()], vec![])
    }

    pub fn iter_left(&self) -> impl Iterator<Item = SurrealFinite> {
        cache_left(self.hash).into_iter()
    }

    pub fn iter_right(&self) -> impl Iterator<Item = SurrealFinite> {
        cache_right(self.hash).into_iter()
    }

    fn left_is_empty(&self) -> bool {
        cache_left(self.hash).is_empty()
    }

    fn right_is_empty(&self) -> bool {
        cache_right(self.hash).is_empty()
    }

    pub fn stof(&self) -> f64 {
        match (self.left_is_empty(), self.right_is_empty()) {
            (true, true) => 0.0,
            (true, false) => self.iter_right().next().unwrap().stof() - 1.0,
            (false, true) => self.iter_left().last().unwrap().stof() + 1.0,
            (false, false) => {
                (self.iter_left().last().unwrap().stof() + self.iter_right().next().unwrap().stof())
                    / 2.0
            }
        }
    }

    // pub fn to_infinite(&self) -> SurrealInfinite {
    //     SurrealInfinite::from_finite(*self)
    // }

    // pub fn to_element(&self) -> SurrealElement {
    //     // lazy?
    //     SurrealElement::Finite(*self)
    // }

    // get birthday of surreal
    // create pseudo surreals? (would break Eq and Ord laws if included in SurrealFinite)
}

impl PartialEq for SurrealFinite {
    fn eq(&self, other: &SurrealFinite) -> bool {
        construction::leq(self, other) && construction::leq(other, self)
    }
}

impl Eq for SurrealFinite {}

impl PartialOrd for SurrealFinite {
    fn partial_cmp(&self, other: &SurrealFinite) -> Option<Ordering> {
        if !construction::leq(self, other) {
            Some(Ordering::Greater)
        } else if !construction::leq(other, self) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Ord for SurrealFinite {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Hash for SurrealFinite {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}

impl ops::Add<SurrealFinite> for SurrealFinite {
    type Output = SurrealFinite;

    fn add(self, other: SurrealFinite) -> SurrealFinite {
        arithmetic::add(self, other)
    }
}

impl ops::AddAssign for SurrealFinite {
    fn add_assign(&mut self, other: SurrealFinite) {
        self.hash = (*self + other).hash;
    }
}

impl ops::Neg for SurrealFinite {
    type Output = SurrealFinite;

    fn neg(self) -> SurrealFinite {
        arithmetic::neg(self)
    }
}

impl ops::Sub<SurrealFinite> for SurrealFinite {
    type Output = SurrealFinite;

    fn sub(self, other: SurrealFinite) -> SurrealFinite {
        self + (-other)
    }
}

impl ops::SubAssign for SurrealFinite {
    fn sub_assign(&mut self, other: SurrealFinite) {
        self.hash = (*self - other).hash;
    }
}

impl ops::Mul<SurrealFinite> for SurrealFinite {
    type Output = SurrealFinite;

    fn mul(self, other: SurrealFinite) -> SurrealFinite {
        arithmetic::mul(self, other)
    }
}

impl ops::MulAssign for SurrealFinite {
    fn mul_assign(&mut self, other: SurrealFinite) {
        self.hash = (*self * other).hash;
    }
}

impl ops::Rem<SurrealFinite> for SurrealFinite {
    type Output = SurrealFinite;

    fn rem(self, other: SurrealFinite) -> SurrealFinite {
        let mut total = self;
        while total >= other {
            total -= other;
        }
        total
    }
}

impl ops::RemAssign for SurrealFinite {
    fn rem_assign(&mut self, other: SurrealFinite) {
        while *self >= other {
            self.hash = (*self - other).hash;
        }
    }
}

impl fmt::Display for SurrealFinite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "< ").unwrap();
        for s in self.iter_left() {
            write!(f, "{} ", s.stof()).unwrap();
        }

        write!(f, "| ").unwrap();

        for s in self.iter_right() {
            write!(f, "{} ", s.stof()).unwrap();
        }
        write!(f, ">").unwrap();

        Ok(())
    }
}
