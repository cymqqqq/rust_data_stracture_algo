use core::cmp::Ordering;
use core::hash::Hash;
use core::hash::Hasher;
use core::iter::FusedIterator;
use core::iter::Iterator;
use core::marker::PhantomData;
use core::ptr::NonNull;
struct Node<T>{
    next:Option<NonNull<Node<T>>>,
    prev:Option<NonNull<Node<T>>>,
    element:T,
}
pub struct TailQ<T>{
    head:Option<NonNull<Node<T>>>,
    tail:Option<NonNull<Node<T>>>,
    len:usize,
    marker:PhantomData<Box<Node<T>>>,
}
impl<T> Default for TailQ<T>{
    #[inline]
    fn default()->Self{
        Self::new()
    }
}
