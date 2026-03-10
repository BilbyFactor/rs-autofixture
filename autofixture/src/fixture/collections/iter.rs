use std::collections::{
    BTreeMap,
    BTreeSet,
    BinaryHeap,
    HashMap,
    HashSet,
    LinkedList,
    VecDeque,
};

use std::hash::Hash;
use std::marker::PhantomData;

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::FixtureBuilder,
};

pub struct CollectionBuilder<'b, C, T> {
    fixture: &'b mut Fixture,
    size: usize,
    _phantom: PhantomData<(C, T)>,
}

impl<'b, C, T> CollectionBuilder<'b, C, T> {
    /// Sets the desired collection size for on create.
    /// Default is `3`.
    pub fn with_size(&mut self, size: usize) -> &mut Self {
        self.size = size;

        self
    }
}

impl<'b, C, T> FixtureBuilder<'b> for CollectionBuilder<'b, C, T>
where
    C: AutoFixture + FromIterator<T>,
    T: AutoFixture,
{
    type F = C;

    fn new(f: &'b mut Fixture) -> Self {
        Self {
            fixture: f,
            size: 3,
            _phantom: PhantomData,
        }
    }

    fn create(&mut self) -> Self::F {
        (0..self.size).map(|_| T::create(self.fixture)).collect()
    }
}

impl<T> AutoFixture for Vec<T>
where
    T: AutoFixture,
{
    type Builder<'b> = CollectionBuilder<'b, Vec<T>, T>;

    fn create(f: &mut Fixture) -> Self {
        CollectionBuilder::<Vec<T>, T>::new(f).create()
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        CollectionBuilder::new(f)
    }
}

impl<T> AutoFixture for VecDeque<T>
where
    T: AutoFixture,
{
    type Builder<'b> = CollectionBuilder<'b, VecDeque<T>, T>;

    fn create(f: &mut Fixture) -> Self {
        CollectionBuilder::<VecDeque<T>, T>::new(f).create()
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        CollectionBuilder::new(f)
    }
}

impl<T> AutoFixture for LinkedList<T>
where
    T: AutoFixture,
{
    type Builder<'b> = CollectionBuilder<'b, LinkedList<T>, T>;

    fn create(f: &mut Fixture) -> Self {
        CollectionBuilder::<LinkedList<T>, T>::new(f).create()
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        CollectionBuilder::new(f)
    }
}

impl<T> AutoFixture for HashSet<T>
where
    T: AutoFixture + Eq + Hash,
{
    type Builder<'b> = CollectionBuilder<'b, HashSet<T>, T>;

    fn create(f: &mut Fixture) -> Self {
        CollectionBuilder::<HashSet<T>, T>::new(f).create()
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        CollectionBuilder::new(f)
    }
}

impl<T> AutoFixture for BTreeSet<T>
where
    T: AutoFixture + Ord,
{
    type Builder<'b> = CollectionBuilder<'b, BTreeSet<T>, T>;

    fn create(f: &mut Fixture) -> Self {
        CollectionBuilder::<BTreeSet<T>, T>::new(f).create()
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        CollectionBuilder::new(f)
    }
}

impl<T> AutoFixture for BinaryHeap<T>
where
    T: AutoFixture + Ord,
{
    type Builder<'b> = CollectionBuilder<'b, BinaryHeap<T>, T>;

    fn create(f: &mut Fixture) -> Self {
        CollectionBuilder::<BinaryHeap<T>, T>::new(f).create()
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        CollectionBuilder::new(f)
    }
}

impl<K, V> AutoFixture for HashMap<K, V>
where
    K: AutoFixture + Eq + Hash,
    V: AutoFixture,
{
    type Builder<'b> = CollectionBuilder<'b, HashMap<K, V>, (K, V)>;

    fn create(f: &mut Fixture) -> Self {
        CollectionBuilder::<HashMap<K, V>, (K, V)>::new(f).create()
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        CollectionBuilder::new(f)
    }
}

impl<K, V> AutoFixture for BTreeMap<K, V>
where
    K: AutoFixture + Ord,
    V: AutoFixture,
{
    type Builder<'b> = CollectionBuilder<'b, BTreeMap<K, V>, (K, V)>;

    fn create(f: &mut Fixture) -> Self {
        CollectionBuilder::<BTreeMap<K, V>, (K, V)>::new(f).create()
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        CollectionBuilder::new(f)
    }
}
