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
    {
    	let it = m.iter();
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
    	for &(k, v) in it{
    		assert_eq!(k +1, v);
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
