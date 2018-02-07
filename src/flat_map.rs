use self::Entry::*;
use std::vec;
use std::vec::Vec;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::iter::{Map, FromIterator};
use std::mem::swap;
use std::ops::Index;
use std::slice;
#[derive(Clone)]
pub struct FlatMap<K, V> {
    v: Vec<(K, V)>,
}

pub enum Entry<'a, K: 'a, V: 'a> {
    Vacant(VacantEntry<'a, K, V>),
    Occupied(OccupiedEntry<'a, K, V>),
}

pub struct VacantEntry<'a, K: 'a, V: 'a> {
    v: &'a mut Vec<(K, V)>,
    key: K,
    index: usize,
}

pub struct OccupiedEntry<'a, K: 'a, V: 'a> {
    v: &'a mut Vec<(K, V)>,
    index: usize,
}

pub struct IntoIter<K, V> {
    inner: vec::IntoIter<(K, V)>,
}

pub struct IterMut<'a, K: 'a, V: 'a> {
    inner: slice::IterMut<'a, (K, V)>,
}

pub struct ValuesMut<'a, K: 'a, V: 'a> {
    inner: IterMut<'a, K, V>,
}

pub struct Iter<'a, K: 'a, V: 'a> {
    inner: slice::Iter<'a, (K, V)>,
}

pub struct Keys<'a, K: 'a, V: 'a> {
    inner: Map<Iter<'a, K, V>, fn((&'a K, &'a V)) -> &'a K>,
}

pub struct Values<'a, K: 'a, V: 'a> {
    inner: Map<Iter<'a, K, V>, fn((&'a K, &'a V)) -> &'a V>,
}

impl<K, V> FlatMap<K, V> {

    pub fn new() -> FlatMap<K, V> {
        FlatMap { v: vec![] }
    }

    pub fn with_capacity(capacity: usize) -> FlatMap<K, V> {
        FlatMap { v: Vec::with_capacity(capacity) }
    }

    /// Returns the number of elements the `VecMap` can hold without
    /// reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use flat_map::FlatMap;
    /// let map: FlatMap<String, String> = FlatMap::with_capacity(10);
    /// assert!(map.capacity() >= 10);
    /// ```
    pub fn capacity(&self) -> usize {
        self.v.capacity()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.v.reserve(additional)
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.v.reserve_exact(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.v.shrink_to_fit()
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }

    /// Return true if the map contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use flat_map::FlatMap;
    ///
    /// let mut a = FlatMap::new();
    /// assert!(a.is_empty());
    /// a.insert("1", "a");
    /// assert!(!a.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    pub fn iter<'r>(&'r self) -> Iter<'r, K, V> {
        Iter { inner: self.v.iter() }
    }

    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut { inner: self.v.iter_mut() }
    }

    pub fn values_mut(&mut self) -> ValuesMut<K, V> {
        ValuesMut { inner: self.iter_mut() }
    }

    pub fn keys<'a>(&'a self) -> Keys<'a, K, V> {
        fn first<A, B>((a, _): (A, B)) -> A {
            a
        }
        let first: fn((&'a K, &'a V)) -> &'a K = first; // coerce to fn pointer
        Keys { inner: self.iter().map(first) }
    }

    pub fn values<'a>(&'a self) -> Values<'a, K, V> {
        fn second<A, B>((_, b): (A, B)) -> B {
            b
        }
        let second: fn((&'a K, &'a V)) -> &'a V = second; // coerce to fn pointer
        Values { inner: self.iter().map(second) }
    }

    pub fn clear(&mut self) {
        self.v.clear()
    }
}

impl<K: Ord, V> FlatMap<K, V> {
    pub fn insert(&mut self, key: K, mut v: V) -> Option<V> {
        match self.v[..].binary_search_by(|&(ref k, _)| k.cmp(&key)) {
            Err(i) => {
                self.v.insert(i, (key, v));
                None
            }
            Ok(i) => {
                let &mut (_, ref mut value) = &mut self.v[i];
                swap(value, &mut v);
                Some(v)
            }
        }
    }

    pub fn append(&mut self, other: &mut Self) {
        self.v.reserve(other.len());
        for (k, v) in other.v.drain(..) {
            self.insert(k, v);
        }
    }

    pub fn split_off(&mut self, key: &K) -> Self {
        match self.v[..].binary_search_by(|&(ref k, _)| k.cmp(key)) {
            Err(_) => Self::new(),
            Ok(at) => {
                let v = self.v.split_off(at);
                FlatMap { v: v }
            }
        }

    }

    pub fn get<Q: ?Sized>(&self, q: &Q) -> Option<&V>
        where K: Borrow<Q>,
              Q: Ord
    {
        match self.v[..].binary_search_by(|&(ref k, _)| k.borrow().cmp(q)) {
            Err(_) => None,
            Ok(idx) => {
                let (_, ref v) = self.v[idx];
                Some(v)
            }
        }
    }

    pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
        where K: Borrow<Q>,
              Q: Ord
    {
        self.get(k).is_some()
    }

    /// Return Option<&mut V>.
    ///
    /// # Example
    ///
    /// ```
    /// use flat_map::FlatMap;
    ///
    /// let mut m = FlatMap::new();
    /// m.insert(1, "foo".to_string());
    /// m.get_mut(&1).unwrap().push_str("bar");
    /// assert_eq!("foobar", m.get_mut(&1).unwrap());
    /// ```
    pub fn get_mut<Q: ?Sized>(&mut self, q: &Q) -> Option<&mut V>
        where K: Borrow<Q>,
              Q: Ord
    {
        match self.v[..].binary_search_by(|&(ref k, _)| k.borrow().cmp(q)) {
            Err(_) => None,
            Ok(idx) => {
                match self.v.get_mut(idx) {
                    Some(&mut (_, ref mut v)) => Some(v),
                    _ => None,
                }
            }
        }
    }

    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        match self.v[..].binary_search_by(|&(ref k, _)| k.cmp(&key)) {
            Err(i) => {
                Vacant(VacantEntry {
                           v: &mut self.v,
                           key: key,
                           index: i,
                       })
            }
            Ok(i) => {
                Occupied(OccupiedEntry {
                             v: &mut self.v,
                             index: i,
                         })
            }
        }
    }

    pub fn remove<Q: ?Sized>(&mut self, q: &Q) -> Option<V>
        where K: Borrow<Q>,
              Q: Ord
    {
        match self.v[..].binary_search_by(|&(ref k, _)| k.borrow().cmp(q)) {
            Err(_) => None,
            Ok(i) => {
                let (_, value) = self.v.remove(i);
                Some(value)
            }
        }
    }

        
}

impl<'a, K: Ord, V> Entry<'a, K, V> {
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Occupied(entry) => entry.into_mut(),
            Vacant(entry) => entry.insert(default),
        }
    }

    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
        match self {
            Occupied(entry) => entry.into_mut(),
            Vacant(entry) => entry.insert(default()),
        }
    }
}

impl<'a, K: Ord, V> VacantEntry<'a, K, V> {
    pub fn insert(self, value: V) -> &'a mut V {
        self.v.insert(self.index, (self.key, value));
        let &mut (_, ref mut value) = &mut self.v[self.index];
        value
    }
}

impl<'a, K: Ord, V> OccupiedEntry<'a, K, V> {
    pub fn key(&self) -> &K {
        let (ref key, _) = self.v[self.index];
        key
    }

    pub fn get(&self) -> &V {
        let (_, ref value) = self.v[self.index];
        value
    }

    pub fn get_mut(&mut self) -> &mut V {
        let (_, ref mut value) = self.v[self.index];
        value
    }

    pub fn into_mut(self) -> &'a mut V {
        let &mut (_, ref mut value) = &mut self.v[self.index];
        value
    }

    pub fn insert(&mut self, mut value: V) -> V {
        let &mut (_, ref mut old_value) = &mut self.v[self.index];
        swap(old_value, &mut value);
        value
    }

    pub fn remove(self) -> V {
        let (_, value) = self.v.remove(self.index);
        value
    }

    
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        match self.inner.next() {
            Some(&(ref k, ref v)) => Some((k, v)),
            None => None,
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K, V> Clone for Iter<'a, K, V> {
    fn clone(&self) -> Iter<'a, K, V> {
        Iter { inner: self.inner.clone() }
    }
}

impl<'a, K, V> DoubleEndedIterator for Iter<'a, K, V> {
    fn next_back(&mut self) -> Option<(&'a K, &'a V)> {
        match self.inner.next_back() {
            Some(&(ref k, ref v)) => Some((k, v)),
            None => None,
        }
    }
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<(&'a K, &'a mut V)> {
        match self.inner.next() {
            Some(&mut (ref k, ref mut v)) => Some((k, v)),
            None => None,
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for IterMut<'a, K, V> {
    fn next_back(&mut self) -> Option<(&'a K, &'a mut V)> {
        match self.inner.next_back() {
            Some(&mut (ref k, ref mut v)) => Some((k, v)),
            None => None,
        }
    }
}

impl<'a, K, V> ExactSizeIterator for IterMut<'a, K, V> {}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        self.inner.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<K, V> DoubleEndedIterator for IntoIter<K, V> {
    fn next_back(&mut self) -> Option<(K, V)> {
        self.inner.next_back()
    }
}

impl<K, V> ExactSizeIterator for IntoIter<K, V> {}

impl<K, V> IntoIterator for FlatMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> IntoIter<K, V> {
        IntoIter { inner: self.v.into_iter() }
    }
}

impl<'a, K, V> IntoIterator for &'a FlatMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Iter<'a, K, V> {
        Iter { inner: self.v.iter() }
    }
}

impl<'a, K, V> IntoIterator for &'a mut FlatMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> IterMut<'a, K, V> {
        IterMut { inner: self.v.iter_mut() }
    }
}

impl<'a, K, V> Clone for Keys<'a, K, V> {
    fn clone(&self) -> Keys<'a, K, V> {
        Keys { inner: self.inner.clone() }
    }
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<(&'a K)> {
        self.inner.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for Keys<'a, K, V> {
    fn next_back(&mut self) -> Option<(&'a K)> {
        self.inner.next_back()
    }
}

impl<'a, K, V> ExactSizeIterator for Keys<'a, K, V> {}

impl<'a, K, V> Clone for Values<'a, K, V> {
    fn clone(&self) -> Values<'a, K, V> {
        Values { inner: self.inner.clone() }
    }
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<(&'a V)> {
        self.inner.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for Values<'a, K, V> {
    fn next_back(&mut self) -> Option<(&'a V)> {
        self.inner.next_back()
    }
}

impl<'a, K, V> ExactSizeIterator for Values<'a, K, V> {}

impl<K: Ord, V> FromIterator<(K, V)> for FlatMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> FlatMap<K, V> {
        let mut vec: Vec<_> = iter.into_iter().collect();
        vec.sort_by(|kv1, kv2| kv1.0.cmp(&kv2.0));
        vec.dedup_by(|kv1, kv2| kv1.0 == kv2.0);
        Self {
            v: vec
        }
    }
}

impl<K: Ord, V> Extend<(K, V)> for FlatMap<K, V> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<'a, K: Ord + Copy, V: Copy> Extend<(&'a K, &'a V)> for FlatMap<K, V> {
    fn extend<I: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: I) {
        self.extend(iter.into_iter().map(|(&key, &value)| (key, value)));
    }
}

impl<K: Hash, V: Hash> Hash for FlatMap<K, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for elt in self {
            elt.hash(state);
        }
    }
}

impl<K: Ord, V> Default for FlatMap<K, V> {
    fn default() -> FlatMap<K, V> {
        FlatMap::new()
    }
}

impl<K: Ord, V: Ord> Ord for FlatMap<K, V> {
    fn cmp(&self, other: &FlatMap<K, V>) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

impl<K: PartialEq, V: PartialEq> PartialEq for FlatMap<K, V> {
    fn eq(&self, other: &FlatMap<K, V>) -> bool {
        self.len() == other.len() && self.iter().zip(other).all(|(a, b)| a == b)
    }
}

impl<K: Eq, V: Eq> Eq for FlatMap<K, V> {}

impl<K: PartialOrd, V: PartialOrd> PartialOrd for FlatMap<K, V> {
    fn partial_cmp(&self, other: &FlatMap<K, V>) -> Option<Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}

impl<K: Debug, V: Debug> Debug for FlatMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<'a, K: Ord, Q: ?Sized, V> Index<&'a Q> for FlatMap<K, V>
    where K: Borrow<Q>,
          Q: Ord
{
    type Output = V;

    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("no entry found for key")
    }
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<&'a mut V> {
        self.inner.next().map(|(_, v)| v)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for ValuesMut<'a, K, V> {
    fn next_back(&mut self) -> Option<&'a mut V> {
        self.inner.next_back().map(|(_, v)| v)
    }
}

impl<'a, K, V> ExactSizeIterator for ValuesMut<'a, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}


#[cfg(feature = "serde1")]
mod serde_impl
{
    // the serde serialization/deserialization is manually handled to
    // serialize the FlatMap as a classic map
    // and not as a vector<K, V>
    // this way the FlatMap is serialized in json as:
    // { "k1": "v1", "k2": "v2" }
    // and not
    // {"v": [["k1", "v1"],["k2", "v2"]]}


    use std::marker::PhantomData;
    use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};
    use serde::{Serialize, Serializer};
    use serde::ser::SerializeMap;
    use super::FlatMap;
    use std::fmt;

    impl<K, V> Serialize for FlatMap<K, V>
    where K: Ord + Serialize, V: Serialize {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut map = serializer.serialize_map(Some(self.len()))?;
            for (k, v) in self {
                map.serialize_entry(k, v)?;
            }
            map.end()
        }
    }


    struct FlatMapVisitor<K, V> {
        marker: PhantomData<fn() -> FlatMap<K, V>>
    }

    impl<K, V> FlatMapVisitor<K, V> {
        fn new() -> Self {
            FlatMapVisitor {
                marker: PhantomData
            }
        }
    }

    impl<'de, K: Ord, V> Visitor<'de> for FlatMapVisitor<K, V>
        where K: Deserialize<'de>,
            V: Deserialize<'de>
    {
        type Value = FlatMap<K, V>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a flat_map")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where M: MapAccess<'de>
        {
            let mut map = FlatMap::with_capacity(access.size_hint().unwrap_or(0));
            while let Some((key, value)) = access.next_entry()? {
                map.insert(key, value);
            }
            Ok(map)
        }
    }

    impl<'de, K: Ord, V> Deserialize<'de> for FlatMap<K, V>
        where K: Deserialize<'de>,
            V: Deserialize<'de>
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: Deserializer<'de>
        {
            deserializer.deserialize_map(FlatMapVisitor::new())
        }
    }
}
