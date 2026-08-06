use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

use rs_autofixture::fixture::Fixture;
use rs_autofixture::fixture::builder::FixtureBuilder;

#[test]
fn vec_creates_with_default_size() {
    let mut f = Fixture::new();
    let v: Vec<u32> = f.create();
    assert_eq!(v.len(), 3);
}

#[test]
fn vec_builder_with_size() {
    let mut f = Fixture::new();

    let v = f
        .build::<Vec<u32>>()
        .with_size(7)
        .create();

    assert_eq!(v.len(), 7);
}

#[test]
fn vec_deque_creates_with_default_size() {
    let mut f = Fixture::new();
    let v: VecDeque<u32> = f.create();
    assert_eq!(v.len(), 3);
}

#[test]
fn vec_deque_builder_with_size() {
    let mut f = Fixture::new();

    let v = f
        .build::<VecDeque<u32>>()
        .with_size(5)
        .create();

    assert_eq!(v.len(), 5);
}

#[test]
fn linked_list_creates_with_default_size() {
    let mut f = Fixture::new();
    let v: LinkedList<u32> = f.create();
    assert_eq!(v.len(), 3);
}

#[test]
fn linked_list_builder_with_size() {
    let mut f = Fixture::new();

    let v = f
        .build::<LinkedList<u32>>()
        .with_size(4)
        .create();

    assert_eq!(v.len(), 4);
}

#[test]
fn hashset_creates_with_default_size() {
    let mut f = Fixture::new();
    let v: HashSet<u32> = f.create();
    assert_eq!(v.len(), 3);
}

#[test]
fn hashset_builder_with_size() {
    let mut f = Fixture::new();

    let v = f
        .build::<HashSet<u32>>()
        .with_size(6)
        .create();

    assert_eq!(v.len(), 6);
}

#[test]
fn btreeset_creates_with_default_size() {
    let mut f = Fixture::new();
    let v: BTreeSet<u32> = f.create();
    assert_eq!(v.len(), 3);
}

#[test]
fn btreeset_builder_with_size() {
    let mut f = Fixture::new();

    let v = f
        .build::<BTreeSet<u32>>()
        .with_size(5)
        .create();

    assert_eq!(v.len(), 5);
}

#[test]
fn binary_heap_creates_with_default_size() {
    let mut f = Fixture::new();
    let v: BinaryHeap<u32> = f.create();
    assert_eq!(v.len(), 3);
}

#[test]
fn binary_heap_builder_with_size() {
    let mut f = Fixture::new();

    let v = f
        .build::<BinaryHeap<u32>>()
        .with_size(8)
        .create();

    assert_eq!(v.len(), 8);
}

#[test]
fn hashmap_creates_with_default_size() {
    let mut f = Fixture::new();
    let v: HashMap<u32, bool> = f.create();
    assert_eq!(v.len(), 3);
}

#[test]
fn hashmap_builder_with_size() {
    let mut f = Fixture::new();

    let v = f
        .build::<HashMap<u32, bool>>()
        .with_size(5)
        .create();

    assert_eq!(v.len(), 5);
}

#[test]
fn btreemap_creates_with_default_size() {
    let mut f = Fixture::new();
    let v: BTreeMap<u32, bool> = f.create();
    assert_eq!(v.len(), 3);
}

#[test]
fn btreemap_builder_with_size() {
    let mut f = Fixture::new();

    let v = f
        .build::<BTreeMap<u32, bool>>()
        .with_size(4)
        .create();

    assert_eq!(v.len(), 4);
}

#[test]
fn tuple_creates_successfully() {
    let mut f = Fixture::new();
    let _pair: (u32, bool) = f.create();
}

#[test]
fn tuple_builder_creates_successfully() {
    let mut f = Fixture::new();
    let mut builder = f.build::<(u32, bool)>();
    let _pair = builder.create();
}

#[test]
fn tuple_values_vary() {
    let mut f = Fixture::new();

    let values: Vec<(u32, u32)> = f
        .create_many(10)
        .collect();

    let all_same = values
        .windows(2)
        .all(|w| w[0] == w[1]);

    assert!(!all_same, "expected tuple values to vary");
}
