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
#[derive(Clone)]
pub struct Iter<'a,T:'a>{
    head:Option<NonNull<Node<T>>>,
    tail:Option<NonNull<Node<T>>>,
    len:usize,
    marker:PhantomData<&'a Node<T>>,
}
pub struct IterMut<'a,T:'a>{
    head:Option<NonNull<Node<T>>>,
    tail:Option<NonNull<Node<T>>>,
    len:usize,
    marker:PhantomData<&'a Node<T>>,
}
impl<T> Node<T>{
    fn new(element:T)->Self{
        Node{
            next:None,
            prev:None,
            element,
        }
    }
    #[allow(clippy::boxed_local)]
    fn into_element(self:Box<Self>)->T{
        self.element
    }
}
