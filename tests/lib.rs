extern crate flat_map;

use flat_map::FlatMap;
use flat_map::Vacant;
use flat_map::Occupied;

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
fn test_clone() {
    let mut map = FlatMap::new();
    let size = 100;
    assert_eq!(map.len(), 0);

    for i in 0..size {
        assert_eq!(map.insert(i, 10*i), None);
        assert_eq!(map.len(), i + 1);
        assert_eq!(map, map.clone());
    }

    for i in 0..size {
        assert_eq!(map.insert(i, 100*i), Some(10*i));
        assert_eq!(map.len(), size);
        assert_eq!(map, map.clone());
    }

    for i in 0..size/2 {
        assert_eq!(map.remove(&(i*2)), Some(i*200));
        assert_eq!(map.len(), size - i - 1);
        assert_eq!(map, map.clone());
    }

    for i in 0..size/2 {
        assert_eq!(map.remove(&(2*i)), None);
        assert_eq!(map.remove(&(2*i+1)), Some(i*200 + 100));
        assert_eq!(map.len(), size/2 - i - 1);
        assert_eq!(map, map.clone());
    }
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
