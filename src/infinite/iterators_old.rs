use super::{SurrealFinite, SurrealInfinite};
use std::fmt;
use std::rc::Rc;

/// Either a finite or an infinite surreal number.
#[derive(Clone)]
pub enum SurrealElement {
    Finite(SurrealFinite),
    Infinite(SurrealInfinite),
}

impl SurrealElement {
    pub fn coerce_finite(&self) -> SurrealFinite {
        match self {
            Self::Finite(s) => *s,
            Self::Infinite(_) => panic!(),
        }
    }
}

impl fmt::Display for SurrealElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Finite(s) => s.stof().to_string(),
                Self::Infinite(s) => match &s.name {
                    Some(n) => n.clone(),
                    _ => s.to_string(),
                },
            }
        )
    }
}

pub trait SurrealIterator {
    fn take(&self, n: usize) -> Vec<SurrealElement>; // memoise these values?
    fn take_fmt(&self, n: usize) -> Vec<String>;
}

impl fmt::Display for dyn SurrealIterator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, x) in self.take_fmt(6).iter().enumerate() {
            if i == 5 {
                write!(f, "... ").unwrap();
            } else {
                write!(f, "{} ", x).unwrap();
            }
        }

        Ok(())
    }
}

pub struct SurrealBasicSet {
    f: Rc<dyn Fn(Option<SurrealElement>, usize) -> Option<SurrealElement>>,
    init: Option<SurrealElement>,
}

impl SurrealBasicSet {
    pub fn new(
        f: Rc<dyn Fn(Option<SurrealElement>, usize) -> Option<SurrealElement>>, // should this only take just finite funcs or just infinite funcs?
        init: Option<SurrealElement>,
    ) -> SurrealBasicSet {
        SurrealBasicSet { f, init }
    }
}

impl SurrealIterator for SurrealBasicSet {
    fn take(&self, n: usize) -> Vec<SurrealElement> {
        let mut taken = Vec::new();
        let mut prev = self.init.clone();

        for i in 0..n {
            let curr = (self.f)(prev.clone(), i);
            match curr.clone() {
                Some(x) => taken.push(x),
                None => break,
            }
            prev = curr;
        }

        taken
    }

    fn take_fmt(&self, n: usize) -> Vec<String> {
        self.take(n).iter().map(|x| x.to_string()).collect()
    }
}

pub struct SurrealZipSet {
    // have to deal w/ duplicates!
    iters: Vec<Rc<dyn SurrealIterator>>,
}

impl SurrealZipSet {
    pub fn new(iters: Vec<Rc<dyn SurrealIterator>>) -> SurrealZipSet {
        SurrealZipSet { iters }
    }

    pub fn new_rc(iters: Vec<Rc<dyn SurrealIterator>>) -> Rc<SurrealZipSet> {
        // is new_rc ugly?
        Rc::new(SurrealZipSet::new(iters))
    }
}

impl SurrealIterator for SurrealZipSet {
    fn take(&self, n: usize) -> Vec<SurrealElement> {
        let mut all_taken = Vec::new();
        for t in &self.iters {
            all_taken.push(t.take(n));
        }

        let mut taken = Vec::new();
        for i in 0..n {
            for t in &all_taken {
                if i < t.len() {
                    taken.push(t[i].clone());
                }
            }
        }

        taken
    }

    fn take_fmt(&self, n: usize) -> Vec<String> {
        self.take(n).iter().map(|x| x.to_string()).collect()
    }
}

pub struct SurrealAddSet {
    lhs: SurrealElement,
    rhs: Rc<dyn SurrealIterator>,
}

impl SurrealAddSet {
    pub fn new(lhs: SurrealElement, rhs: Rc<dyn SurrealIterator>) -> SurrealAddSet {
        SurrealAddSet { lhs, rhs }
    }

    pub fn new_rc(lhs: SurrealElement, rhs: Rc<dyn SurrealIterator>) -> Rc<SurrealAddSet> {
        Rc::new(SurrealAddSet::new(lhs, rhs))
    }
}

impl SurrealIterator for SurrealAddSet {
    fn take(&self, n: usize) -> Vec<SurrealElement> {
        let mut taken = Vec::new();

        for i in self.rhs.take(n) {
            taken.push(match (i, self.lhs.clone()) {
                (SurrealElement::Finite(x), SurrealElement::Finite(y)) => (x + y).to_element(),
                (SurrealElement::Finite(x), SurrealElement::Infinite(y)) => match y.value {
                    Some(s) => (x + s).to_element(),
                    None => (x.to_infinite() + y).to_element(),
                },
                (SurrealElement::Infinite(x), SurrealElement::Finite(y)) => match x.value {
                    Some(s) => (s + y).to_element(),
                    None => (x + y.to_infinite()).to_element(),
                },
                (SurrealElement::Infinite(x), SurrealElement::Infinite(y)) => {
                    match (x.value, y.value) {
                        (Some(a), Some(b)) => (a + b).to_element(),
                        _ => (x + y).to_element(),
                    }
                }
            });
        }

        taken
    }

    fn take_fmt(&self, n: usize) -> Vec<String> {
        let mut taken = Vec::new();

        for i in self.rhs.take(n) {
            taken.push(format!("({} + {})", i, self.lhs.clone()));
        }

        taken
    }
}

pub struct SurrealNegSet {
    iter: Rc<dyn SurrealIterator>,
}

impl SurrealNegSet {
    pub fn new(iter: Rc<dyn SurrealIterator>) -> SurrealNegSet {
        SurrealNegSet { iter }
    }

    pub fn new_rc(iter: Rc<dyn SurrealIterator>) -> Rc<SurrealNegSet> {
        Rc::new(SurrealNegSet::new(iter))
    }
}

impl SurrealIterator for SurrealNegSet {
    fn take(&self, n: usize) -> Vec<SurrealElement> {
        let mut taken = Vec::new();

        for i in self.iter.take(n) {
            taken.push(match i {
                SurrealElement::Finite(s) => (-s).to_element(),
                SurrealElement::Infinite(s) => (-s).to_element(),
            })
        }

        taken
    }

    fn take_fmt(&self, n: usize) -> Vec<String> {
        let mut taken = Vec::new();

        for i in self.iter.take(n) {
            taken.push(format!("-{}", i))
        }

        taken
    }
}
