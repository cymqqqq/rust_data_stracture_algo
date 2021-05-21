use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash,Hasher};
const INITIAL_NBUCKETS:usize=1;

pub struct HashMap<K,V>{
    buckets:Vec<Vec<(K,V)>>,
    item:usize,
}
impl<K,V> HashMap<K,V>{
    pub fn new()->Self{
        HashMap{
            buckets:Vec::new(),
            item:0,
        }
    }
}
pub struct OccupiedEntry<'a, K: 'a, V: 'a> {
    entry: &'a mut (K, V),
}

pub struct VacantEntry<'a, K: 'a, V: 'a> {
    key: K,
    map: &'a mut HashMap<K, V>,
    bucket: usize,
}

impl<'a, K: 'a, V: 'a> VacantEntry<'a, K, V> {
    pub fn insert(self, value: V) -> &'a mut V
    where
        K: Hash + Eq,
    {
        self.map.buckets[self.bucket].push((self.key, value));
        self.map.items += 1;
        &mut self.map.buckets[self.bucket].last_mut().unwrap().1
    }
}
pub enum Entry<'a, K: 'a, V: 'a> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}
//TODO
