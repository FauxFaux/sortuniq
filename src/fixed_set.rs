use std::borrow::Borrow;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;

pub struct FixedSet<T> {
    size: usize,
    inner: HashSet<T>,
    written: VecDeque<T>,
}

impl<T: Hash + Eq + Clone> FixedSet<T> {
    pub fn new(size: usize) -> FixedSet<T> {
        FixedSet {
            size,
            inner: HashSet::with_capacity(size),
            written: VecDeque::with_capacity(size),
        }
    }

    pub fn contains<Q>(&self, val: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.inner.contains(&val)
    }

    pub fn insert(&mut self, val: T) {
        self.inner.insert(val.clone());
        if self.written.len() >= self.size {
            assert_eq!(self.written.len(), self.size);
            let oldest = self.written.pop_front().expect("must exist");
            self.inner.remove(&oldest);
        }
        self.written.push_back(val);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fixed() {
        let mut f = super::FixedSet::new(3);
        f.insert("1");
        assert!(f.contains(&"1"));
        f.insert("2");
        assert!(f.contains(&"1"));
        assert!(f.contains(&"2"));
        f.insert("3");
        assert!(f.contains(&"1"));
        assert!(f.contains(&"2"));
        assert!(f.contains(&"3"));
        f.insert("4");
        assert!(f.contains(&"2"));
        assert!(f.contains(&"3"));
        assert!(f.contains(&"4"));
    }
}
