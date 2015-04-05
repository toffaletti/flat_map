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
    assert_eq!(m.iter().last(), Some(&(0, 2)));
    assert_eq!(m.remove(&0), Some(2));
    m.insert(0, 1);
    assert!(!m.is_empty());
    m.clear();
    assert!(m.is_empty());

    m.insert(0, 0);
    m.insert(1, 1);
    m.insert(2, 2);
    assert_eq!(m.iter().count(), 3);
    let mut it = m.iter();
    it.next();
    assert_eq!(it.count(), 2);
}
