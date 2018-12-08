#![feature(test)]

extern crate flat_map;
extern crate rand;
extern crate test;

use flat_map::FlatMap;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;
use std::collections::BTreeMap;
use std::iter::FromIterator;
use test::Bencher;

type Key = u64;
type Value = u64;

const NUM_KEYS: usize = 2 << 12;
const NUM_VALUES: usize = 1 << 12;

fn flat_map_setup() -> (FlatMap<Key, Value>, Vec<Key>) {
    let num_items = 1 << 25;
    let mut keys = Vec::new();
    let mut key_vals = Vec::with_capacity(num_items);
    let mut rng = rand::IsaacRng::new_unseeded();
    for i in 0..NUM_KEYS * NUM_VALUES {
        keys.push(i as Key);
        let value = rng.gen::<Value>();
        key_vals.push((i as Key, value))
    }
    let map: FlatMap<_, _> = key_vals.into_iter().collect();
    (map, keys)
}

fn btree_map_setup() -> (BTreeMap<Key, Value>, Vec<Key>) {
    let num_items = 1 << 25;
    let mut keys = Vec::new();
    let mut key_vals = Vec::with_capacity(num_items);
    let mut rng = rand::IsaacRng::new_unseeded();
    for i in 0..NUM_KEYS * NUM_VALUES {
        keys.push(i as Key);
        let value = rng.gen::<Value>();
        key_vals.push((i as Value, value))
    }
    let map = BTreeMap::from_iter(key_vals);
    (map, keys)
}

#[bench]
fn bench_flat_map_from_iter(b: &mut Bencher) {
    let num_keys = 1 << 10;
    let num_values = 1 << 10;
    let mut keys = Vec::new();
    let mut key_vals = Vec::with_capacity(num_keys * num_values);
    let mut rng = rand::IsaacRng::new_unseeded();
    for i in 0..num_keys * num_values {
        keys.push(i as Key);
        let value = rng.gen::<Value>();
        key_vals.push((i as Key, value))
    }
    b.iter(|| {
        let map = FlatMap::from_iter(key_vals.clone());
        map
    })
}

#[bench]
fn bench_flat_map_insert(b: &mut Bencher) {
    let (mut map, keys) = flat_map_setup();
    let mut rng = rand::IsaacRng::new_unseeded();
    let between = Range::new(0, keys.len());
    b.iter(|| {
        let i = between.ind_sample(&mut rng);
        map.insert(keys[i], rng.gen::<Value>());
    })
}

#[bench]
fn bench_flat_map_get(b: &mut Bencher) {
    let (map, keys) = flat_map_setup();
    let mut rng = rand::IsaacRng::new_unseeded();
    let between = Range::new(0, keys.len());
    b.iter(|| {
        let i = between.ind_sample(&mut rng);
        map.get(&keys[i])
    })
}

#[bench]
fn bench_btree_map_from_iter(b: &mut Bencher) {
    let num_keys = 1 << 10;
    let num_values = 1 << 10;
    let mut keys = Vec::new();
    let mut key_vals = Vec::with_capacity(num_keys * num_values);
    let mut rng = rand::IsaacRng::new_unseeded();
    for i in 0..num_keys * num_values {
        keys.push(i as Key);
        let value = rng.gen::<Value>();
        key_vals.push((i as Key, value))
    }
    b.iter(|| {
        let map = BTreeMap::from_iter(key_vals.clone());
        map
    })
}

#[bench]
fn bench_btree_map_insert(b: &mut Bencher) {
    let (mut map, keys) = btree_map_setup();
    let mut rng = rand::IsaacRng::new_unseeded();
    let between = Range::new(0, keys.len());
    b.iter(|| {
        let i = between.ind_sample(&mut rng);
        map.insert(keys[i], rng.gen::<Value>());
    })
}

#[bench]
fn bench_btree_map_get(b: &mut Bencher) {
    let (map, keys) = btree_map_setup();
    let mut rng = rand::IsaacRng::new_unseeded();
    let between = Range::new(0, keys.len());
    b.iter(|| {
        let i = between.ind_sample(&mut rng);
        map.get(&keys[i])
    })
}
