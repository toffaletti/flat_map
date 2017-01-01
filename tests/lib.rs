extern crate flat_map;

use flat_map::FlatMap;
use flat_map::Vacant;
use flat_map::Occupied;

use std::rc::Rc;
use std::iter::FromIterator;

#[test]
fn it_works() {
    let mut m: FlatMap<i32, i32> = FlatMap::new();
    assert!(m.is_empty());
    m.insert(0, 1);
    assert!(m.contains_key(&0));
    assert_eq!(m.get(&0), Some(&1));
    m.insert(0, 2);
    assert_eq!(m.get(&0), Some(&2));
    m.entry(0).or_insert(3);
    assert_eq!(m.get(&0), Some(&2));
    m.entry(1).or_insert(3);
    assert_eq!(m.get(&1), Some(&3));
    assert_eq!(m.remove(&42), None);
    match m.entry(1) {
        Vacant(_) => assert!(false),
        Occupied(entry) => assert_eq!(entry.remove(), 3),
    }
    assert_eq!(m.iter().last(), Some((&0, &2)));
    assert_eq!(m.remove(&0), Some(2));
    m.insert(0, 1);
    assert!(!m.is_empty());
    m.clear();
    assert!(m.is_empty());

    m.insert(0, 0);
    m.insert(1, 1);
    m.insert(2, 2);
    assert_eq!(m.iter().count(), 3);
    {
    	let mut it = m.iter();
    	it.next();
    	assert_eq!(it.count(), 2);
    }
    m.clear();
    
    
    m.insert(0, 0);
    m.insert(1, 1);
    m.insert(2, 2);
    assert_eq!(m.get_mut(&0), Some(&mut 0));
    assert_eq!(m.get_mut(&1), Some(&mut 1));
    assert_eq!(m.get_mut(&2), Some(&mut 2));
    *m.get_mut(&0).unwrap() = *m.get_mut(&0).unwrap() +1;
    *m.get_mut(&1).unwrap() = *m.get_mut(&1).unwrap() +1;
    *m.get_mut(&2).unwrap() = 3;
    {
        let it = m.iter();
        for (k, v) in it {
            assert_eq!(k+1, *v);
        }
    }
    
    let mut m = FlatMap::new();
    m.insert("1", "a");
    m.insert("2", "b");
    *m.get_mut(&"1").unwrap() = "1";
    *m.get_mut(&"2").unwrap() = "2";
    assert_eq!("1".to_string(), m.get(&"1").unwrap().to_string());
    assert_eq!("2".to_string(), m.get(&"2").unwrap().to_string());
   
    let mut m = FlatMap::new(); 
    m.insert(1, "foo".to_string());
    m.insert(2, "ta".to_string());
   	m.get_mut(&1).unwrap().push_str("bar");
   	m.get_mut(&2).unwrap().push_str("da");
	assert_eq!("foobar", m.get_mut(&1).unwrap());
	assert_eq!("tada", m.get_mut(&2).unwrap()); 	
}

#[test]
fn borrow() {
    let mut m: FlatMap<String,String> = FlatMap::new();
    m.insert("Key".to_string(), "Value".to_string());
    assert_eq!(m.get("Key"), Some(&"Value".to_string()));
}

#[test]
fn test_basic_large() {
    let mut map = FlatMap::new();
    let size = 10000;
    assert_eq!(map.len(), 0);

    for i in 0..size {
        assert_eq!(map.insert(i, 10*i), None);
        assert_eq!(map.len(), i + 1);
    }

    for i in 0..size {
        assert_eq!(map.get(&i).unwrap(), &(i*10));
    }

    for i in size..size*2 {
        assert_eq!(map.get(&i), None);
    }

    for i in 0..size {
        assert_eq!(map.insert(i, 100*i), Some(10*i));
        assert_eq!(map.len(), size);
    }

    for i in 0..size {
        assert_eq!(map.get(&i).unwrap(), &(i*100));
    }

    for i in 0..size/2 {
        assert_eq!(map.remove(&(i*2)), Some(i*200));
        assert_eq!(map.len(), size - i - 1);
    }

    for i in 0..size/2 {
        assert_eq!(map.get(&(2*i)), None);
        assert_eq!(map.get(&(2*i+1)).unwrap(), &(i*200 + 100));
    }

    for i in 0..size/2 {
        assert_eq!(map.remove(&(2*i)), None);
        assert_eq!(map.remove(&(2*i+1)), Some(i*200 + 100));
        assert_eq!(map.len(), size/2 - i - 1);
    }
}

#[test]
fn test_basic_small() {
    let mut map = FlatMap::new();
    assert_eq!(map.remove(&1), None);
    assert_eq!(map.get(&1), None);
    assert_eq!(map.insert(1, 1), None);
    assert_eq!(map.get(&1), Some(&1));
    assert_eq!(map.insert(1, 2), Some(1));
    assert_eq!(map.get(&1), Some(&2));
    assert_eq!(map.insert(2, 4), None);
    assert_eq!(map.get(&2), Some(&4));
    assert_eq!(map.remove(&1), Some(2));
    assert_eq!(map.remove(&2), Some(4));
    assert_eq!(map.remove(&1), None);
}

#[test]
fn test_iter() {
    let size = 10000;

    // Forwards
    let mut map: FlatMap<_, _> = (0..size).map(|i| (i, i)).collect();

    fn test<T>(size: usize, mut iter: T) where T: Iterator<Item=(usize, usize)> {
        for i in 0..size {
            assert_eq!(iter.size_hint(), (size - i, Some(size - i)));
            assert_eq!(iter.next().unwrap(), (i, i));
        }
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
    }
    test(size, map.iter().map(|(&k, &v)| (k, v)));
    test(size, map.iter_mut().map(|(&k, &mut v)| (k, v)));
    test(size, map.into_iter());
}

#[test]
fn test_iter_rev() {
    let size = 10000;

    // Forwards
    let mut map: FlatMap<_, _> = (0..size).map(|i| (i, i)).collect();

    fn test<T>(size: usize, mut iter: T) where T: Iterator<Item=(usize, usize)> {
        for i in 0..size {
            assert_eq!(iter.size_hint(), (size - i, Some(size - i)));
            assert_eq!(iter.next().unwrap(), (size - i - 1, size - i - 1));
        }
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
    }
    test(size, map.iter().rev().map(|(&k, &v)| (k, v)));
    test(size, map.iter_mut().rev().map(|(&k, &mut v)| (k, v)));
    test(size, map.into_iter().rev());
}

#[test]
fn test_iter_mixed() {
    let size = 10000;

    // Forwards
    let mut map: FlatMap<_, _> = (0..size).map(|i| (i, i)).collect();

    fn test<T>(size: usize, mut iter: T)
            where T: Iterator<Item=(usize, usize)> + DoubleEndedIterator {
        for i in 0..size / 4 {
            assert_eq!(iter.size_hint(), (size - i * 2, Some(size - i * 2)));
            assert_eq!(iter.next().unwrap(), (i, i));
            assert_eq!(iter.next_back().unwrap(), (size - i - 1, size - i - 1));
        }
        for i in size / 4..size * 3 / 4 {
            assert_eq!(iter.size_hint(), (size * 3 / 4 - i, Some(size * 3 / 4 - i)));
            assert_eq!(iter.next().unwrap(), (i, i));
        }
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
    }
    test(size, map.iter().map(|(&k, &v)| (k, v)));
    test(size, map.iter_mut().map(|(&k, &mut v)| (k, v)));
    test(size, map.into_iter());
}

#[test]
fn test_borrow() {
    // make sure these compile -- using the Borrow trait
    {
        let mut map = FlatMap::new();
        map.insert("0".to_string(), 1);
        assert_eq!(map["0"], 1);
    }

    {
        let mut map = FlatMap::new();
        map.insert(Box::new(0), 1);
        assert_eq!(map[&0], 1);
    }

    {
        let mut map = FlatMap::new();
        map.insert(Box::new([0, 1]) as Box<[i32]>, 1);
        assert_eq!(map[&[0, 1][..]], 1);
    }

    {
        let mut map = FlatMap::new();
        map.insert(Rc::new(0), 1);
        assert_eq!(map[&0], 1);
    }
}

#[test]
fn test_entry() {
    let xs = [(1, 10), (2, 20), (3, 30), (4, 40), (5, 50), (6, 60)];

    let mut map: FlatMap<_, _> = xs.iter().cloned().collect();

    // Existing key (insert)
    match map.entry(1) {
        Vacant(_) => unreachable!(),
        Occupied(mut view) => {
            assert_eq!(view.get(), &10);
            assert_eq!(view.insert(100), 10);
        }
    }
    assert_eq!(map.get(&1).unwrap(), &100);
    assert_eq!(map.len(), 6);


    // Existing key (update)
    match map.entry(2) {
        Vacant(_) => unreachable!(),
        Occupied(mut view) => {
            let v = view.get_mut();
            *v *= 10;
        }
    }
    assert_eq!(map.get(&2).unwrap(), &200);
    assert_eq!(map.len(), 6);

    // Existing key (take)
    match map.entry(3) {
        Vacant(_) => unreachable!(),
        Occupied(view) => {
            assert_eq!(view.remove(), 30);
        }
    }
    assert_eq!(map.get(&3), None);
    assert_eq!(map.len(), 5);


    // Inexistent key (insert)
    match map.entry(10) {
        Occupied(_) => unreachable!(),
        Vacant(view) => {
            assert_eq!(*view.insert(1000), 1000);
        }
    }
    assert_eq!(map.get(&10).unwrap(), &1000);
    assert_eq!(map.len(), 6);
}

#[test]
fn test_extend_ref() {
    let mut a = FlatMap::new();
    a.insert(1, "one");
    let mut b = FlatMap::new();
    b.insert(2, "two");
    b.insert(3, "three");

    a.extend(&b);

    assert_eq!(a.len(), 3);
    assert_eq!(a[&1], "one");
    assert_eq!(a[&2], "two");
    assert_eq!(a[&3], "three");
}

#[test]
fn test_zst() {
    let mut m = FlatMap::new();
    assert_eq!(m.len(), 0);

    assert_eq!(m.insert((), ()), None);
    assert_eq!(m.len(), 1);

    assert_eq!(m.insert((), ()), Some(()));
    assert_eq!(m.len(), 1);
    assert_eq!(m.iter().count(), 1);

    m.clear();
    assert_eq!(m.len(), 0);

    for _ in 0..100 {
        m.insert((), ());
    }

    assert_eq!(m.len(), 1);
    assert_eq!(m.iter().count(), 1);
}

// This test's only purpose is to ensure that zero-sized keys with nonsensical orderings
// do not cause segfaults when used with zero-sized values. All other map behavior is
// undefined.
#[test]
fn test_bad_zst() {
    use std::cmp::Ordering;

    struct Bad;

    impl PartialEq for Bad {
        fn eq(&self, _: &Self) -> bool {
            false
        }
    }

    impl Eq for Bad {}

    impl PartialOrd for Bad {
        fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
            Some(Ordering::Less)
        }
    }

    impl Ord for Bad {
        fn cmp(&self, _: &Self) -> Ordering {
            Ordering::Less
        }
    }

    let mut m = FlatMap::new();

    for _ in 0..100 {
        m.insert(Bad, Bad);
    }
}

#[test]
fn test_clone() {
    let mut map = FlatMap::new();
    let size = 100;
    assert_eq!(map.len(), 0);

    for i in 0..size {
        assert_eq!(map.insert(i, 10 * i), None);
        assert_eq!(map.len(), i + 1);
        assert_eq!(map, map.clone());
    }

    for i in 0..size {
        assert_eq!(map.insert(i, 100 * i), Some(10 * i));
        assert_eq!(map.len(), size);
        assert_eq!(map, map.clone());
    }

    for i in 0..size / 2 {
        assert_eq!(map.remove(&(i * 2)), Some(i * 200));
        assert_eq!(map.len(), size - i - 1);
        assert_eq!(map, map.clone());
    }

    for i in 0..size / 2 {
        assert_eq!(map.remove(&(2 * i)), None);
        assert_eq!(map.remove(&(2 * i + 1)), Some(i * 200 + 100));
        assert_eq!(map.len(), size / 2 - i - 1);
        assert_eq!(map, map.clone());
    }
}

macro_rules! create_append_test {
    ($name:ident, $len:expr) => {
        #[test]
        fn $name() {
            let mut a = FlatMap::new();
            for i in 0..8 {
                a.insert(i, i);
            }

            let mut b = FlatMap::new();
            for i in 5..$len {
                b.insert(i, 2*i);
            }

            a.append(&mut b);

            assert_eq!(a.len(), $len);
            assert_eq!(b.len(), 0);

            for i in 0..$len {
                if i < 5 {
                    assert_eq!(a[&i], i);
                } else {
                    assert_eq!(a[&i], 2*i);
                }
            }

            assert_eq!(a.remove(&($len-1)), Some(2*($len-1)));
            assert_eq!(a.insert($len-1, 20), None);
        }
    };
}

// These are mostly for testing the algorithm that "fixes" the right edge after insertion.
// Single node.
create_append_test!(test_append_9, 9);
// Two leafs that don't need fixing.
create_append_test!(test_append_17, 17);
// Two leafs where the second one ends up underfull and needs stealing at the end.
create_append_test!(test_append_14, 14);
// Two leafs where the second one ends up empty because the insertion finished at the root.
create_append_test!(test_append_12, 12);
// Three levels; insertion finished at the root.
create_append_test!(test_append_144, 144);
// Three levels; insertion finished at leaf while there is an empty node on the second level.
create_append_test!(test_append_145, 145);
// Tests for several randomly chosen sizes.
create_append_test!(test_append_170, 170);
create_append_test!(test_append_181, 181);
create_append_test!(test_append_239, 239);
create_append_test!(test_append_1700, 1700);

struct DeterministicRng {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl DeterministicRng {
    fn new() -> Self {
        DeterministicRng {
            x: 0x193a6754,
            y: 0xa8a7d469,
            z: 0x97830e05,
            w: 0x113ba7bb,
        }
    }

    fn next(&mut self) -> u32 {
        let x = self.x;
        let t = x ^ (x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        let w_ = self.w;
        self.w = w_ ^ (w_ >> 19) ^ (t ^ (t >> 8));
        self.w
    }
}

fn rand_data(len: usize) -> Vec<(u32, u32)> {
    let mut rng = DeterministicRng::new();
    Vec::from_iter((0..len).map(|_| (rng.next(), rng.next())))
}

#[test]
fn test_split_off_empty_right() {
    let mut data = rand_data(173);

    let mut map = FlatMap::from_iter(data.clone());
    let right = map.split_off(&(data.iter().max().unwrap().0 + 1));

    data.sort();
    assert!(map.into_iter().eq(data));
    assert!(right.into_iter().eq(None));
}

#[test]
fn test_split_off_empty_left() {
    let mut data = rand_data(314);

    let mut map = FlatMap::from_iter(data.clone());
    let right = map.split_off(&data.iter().min().unwrap().0);

    data.sort();
    assert!(map.into_iter().eq(None));
    assert!(right.into_iter().eq(data));
}

#[test]
fn test_split_off_large_random_sorted() {
    let mut data = rand_data(1529);
    // special case with maximum height.
    data.sort();

    let mut map = FlatMap::from_iter(data.clone());
    let key = data[data.len() / 2].0;
    let right = map.split_off(&key);

    assert!(map.into_iter().eq(data.clone().into_iter().filter(|x| x.0 < key)));
    assert!(right.into_iter().eq(data.into_iter().filter(|x| x.0 >= key)));
}

