use crate::setref::multiple::RefMulti;
use core::hash::{BuildHasher, Hash};

pub struct OwningIter<K, S> {
    inner: crate::iter::OwningIter<K, (), S>,
}

impl<K: Eq + Hash, S: BuildHasher + Clone> OwningIter<K, S> {
    pub(crate) fn new(inner: crate::iter::OwningIter<K, (), S>) -> Self {
        Self { inner }
    }
}

impl<K: Eq + Hash, S: BuildHasher + Clone> Iterator for OwningIter<K, S> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, _)| k)
    }
}

unsafe impl<K, S> Send for OwningIter<K, S>
where
    K: Eq + Hash + Send,
    S: BuildHasher + Clone + Send,
{
}

unsafe impl<K, S> Sync for OwningIter<K, S>
where
    K: Eq + Hash + Sync,
    S: BuildHasher + Clone + Sync,
{
}

pub struct Iter<'a, K, S> {
    inner: crate::iter::Iter<'a, K, (), S>,
}

unsafe impl<'a, 'i, K, S> Send for Iter<'i, K, S>
where
    K: 'a + Eq + Hash + Send,
    S: 'a + BuildHasher + Clone,
{
}

unsafe impl<'a, 'i, K, S> Sync for Iter<'i, K, S>
where
    K: 'a + Eq + Hash + Sync,
    S: 'a + BuildHasher + Clone,
{
}

impl<'a, K: Eq + Hash, S: 'a + BuildHasher + Clone> Iter<'a, K, S> {
    pub(crate) fn new(inner: crate::iter::Iter<'a, K, (), S>) -> Self {
        Self { inner }
    }
}

impl<'a, K: Eq + Hash, S: 'a + BuildHasher + Clone> Iterator
    for Iter<'a, K, S>
{
    type Item = RefMulti<'a, K, S>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(RefMulti::new)
    }
}
