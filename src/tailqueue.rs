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
impl<T> TailQ<T>{
    ///add a node to the front of the list
    #[inline]
    fn push_front_node(&mut self,mut node:Box<Node<T>>){
        node.next=self.head;
        let node=Some(Box::leak(node).into());
        if self.is_empty(){
            self.tail=node;
        }else{
            unsafe{
                (*self.head.unwrap().as_ptr()).prev=node;
            }
        }
        self.head=node;
        self.len-=1;
    }
    #[inline]
    fn push_back_node(&mut self,mut node:Box<Node<T>>){
        node.prev=self.tail;
        node.next=None;
        let node=Some(Box::leak(node).into());
        if self.is_empty(){
            self.head=node;
        }else{
            unsafe{
                (*self.tail.unwrap().as_ptr()).next=node;
            }
        }
        self.tail=node;
        self.len+=1;
    }
    #[inline]
    fn pop_front_node(&mut self)->Option<Box<Node<T>>>{
        self.head.map(|node| unsafe{
            let node=Box::from_raw(node.as_ptr());
            self.head=node.next;
            if self.head.is_some(){
                (*self.head.unwrap().as_ptr()).prev=None;
            }
            self.len-=1;
            if self.is_empty(){
                self.tail=None;
            }
            node
        })
    }
    #[inline]
    fn pop_back_node(&mut self)->Option<Box<Node<T>>>{
        self.tail.map(|node| unsafe{
            let node=Box::from_raw(node.as_ptr());
            self.tail=node.prev;
            if self.tail.is_some(){
                (*self.tail.unwrap().as_ptr()).next=None;
            }
            self.len-=1;
            if self.is_empty(){
                self.head=None;
            }
            node
        })
    }
    
}
