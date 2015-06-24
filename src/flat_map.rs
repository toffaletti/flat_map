use self::Entry::*;

use std::slice;
use std::mem::swap;
use std::borrow::Borrow;

pub struct FlatMap<K, V> {
    v: Vec<(K, V)>,
}

pub enum Entry<'a, K:'a, V:'a> {
    Vacant(VacantEntry<'a, K, V>),
    Occupied(OccupiedEntry<'a, K, V>),
}

pub struct VacantEntry<'a, K:'a, V:'a> {
    v: &'a mut Vec<(K, V)>,
    key: K,
    index: usize,
}

pub struct OccupiedEntry<'a, K:'a, V:'a> {
    v: &'a mut Vec<(K, V)>,
    index: usize,
}

pub struct Iter<'a, K:'a, V:'a> {
    iter: slice::Iter<'a, (K, V)>
}

impl <K: Ord, V> FlatMap<K, V> {
    pub fn new() -> FlatMap<K, V> { FlatMap{ v: vec![] } }

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

    pub fn reserve_len(&mut self, len: usize) {
        let cur_len = self.v.len();
        if len >= cur_len {
            self.v.reserve(len - cur_len);
        }
    }

    pub fn reserve_len_exact(&mut self, len: usize) {
        let cur_len = self.v.len();
        if len >= cur_len {
            self.v.reserve_exact(len - cur_len);
        }
    }

    pub fn len(&self) -> usize { self.v.len() }

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
    pub fn is_empty(&self) -> bool { self.v.is_empty() }

    pub fn insert(&mut self, key: K, mut v: V) -> Option<V> {
        match self.v[..].binary_search_by(|&(ref k, _)| { k.cmp(&key) }) {
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

    pub fn get<Q: ?Sized>(&self, q: &Q) -> Option<&V>
        where K: Borrow<Q>, Q: Ord
    {
        match self.v[..].binary_search_by(|&(ref k, _)| { k.borrow().cmp(&q) }) {
            Err(_) => None,
            Ok(idx) => {
                let (_, ref v) = self.v[idx];
                Some(v)
            }
        }
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
        where K: Borrow<Q>, Q: Ord
    {
	match self.v[..].binary_search_by(|&(ref k, _)| { k.borrow().cmp(&q) }) {
	    Err(_) => None,
	    Ok(idx) => {
		match self.v.get_mut(idx) {
		    Some(&mut (_, ref mut v)) => Some(v),
		    _ => None,
		}
	    }
	}
    }

    pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
        where K: Borrow<Q>, Q: Ord
    {
        self.get(k).is_some()
    }

    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        match self.v[..].binary_search_by(|&(ref k, _)| { k.cmp(&key) }) {
            Err(i) => Vacant(VacantEntry{v: &mut self.v, key: key, index: i}),
            Ok(i) => Occupied(OccupiedEntry{v: &mut self.v, index: i}),
        }
    }

    pub fn remove<Q: ?Sized>(&mut self, q: &Q) -> Option<V>
        where K: Borrow<Q>, Q: Ord
    {
        match self.v[..].binary_search_by(|&(ref k, _)| { k.borrow().cmp(&q) }) {
            Err(_) => None,
            Ok(i) => {
                let (_, value) = self.v.remove(i);
                Some(value)
            }
        }
    }

    pub fn iter<'r>(&'r self) -> Iter<'r, K, V> {
        Iter {
            iter: self.v.iter(),
        }
    }

    pub fn clear(&mut self) {
        self.v.clear()
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
    pub fn insert(mut self, value: V) -> &'a mut V {
        self.v.insert(self.index, (self.key, value));
        let &mut (_, ref mut value) = &mut self.v[self.index];
        value
    }
}

impl<'a, K: Ord, V> OccupiedEntry<'a, K, V> {
    pub fn get(&self) -> &V {
        let (_, ref value) = self.v[self.index];
        value
    }

    pub fn into_mut(self) -> &'a mut V {
        let &mut (_, ref mut value) = &mut self.v[self.index];
        value
    }

    pub fn remove(self) -> V {
        let (_, value) = self.v.remove(self.index);
        value
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = &'a (K, V);

    #[inline]
    fn next(&mut self) -> Option<&'a (K, V)> {
        self.iter.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
