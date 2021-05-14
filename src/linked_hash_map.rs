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
