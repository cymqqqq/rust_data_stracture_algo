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
pub struct List<T>{
    head:Option<NonNull<Node<T>>>,
    len:usize,
    marker:PhantomData<Box<Node<T>>>,
    
}
impl<T> Default for List<T>{
    ///create an empty list
    #[inline]
    fn default()->Self{
        Self::new()
    }
    
}
#[derive(Clone)]
pub struct Iter<'a,T:'a>{
    head:Option<NonNull<Node<T>>>,
    len:usize,
    marker:PhantomData<&'a Node<T>>,
}
pub struct IterMut<'a,T:'a>{
    head:Option<NonNull<Node<T>>>,
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
impl<T> List<T>{
    ///add a node to the front of the list
    #[inline]
    fn push_front_node(&mut self,mut node:Box<Node<T>>){
        node.next=self.head;
        let node=Some(Box::leak(node).into());
        if !self.is_empty(){
            unsafe{
                (*self.head.unwrap().as_ptr()).prev=node;
            }
        }
        self.head=node;
        self.len+=1;
    }
    ///removes a node from the front of the list
    #[inline]
    fn pop_front_node(&mut self)->Option<Box<Node<T>>>{
        self.head.map(|node| unsafe{
            let node=Box::from_raw(node.as_ptr());
            self.head=node.next;
            if self.head.is_some(){
                (*self.head.unwrap().as_ptr()).prev=None;
            }
            self.len-=1;
            node
        })
    }
}
impl<T> List<T> {
    #[inline]
    pub const fn new()->Self{
        Self{
            head:None,
            len:0,
            marker:PhantomData,
        }
    }
    #[inline]
    pub fn iter(&self)->Iter<'_,T>{
        Iter{
            head:self.head,
            len:self.len,
            marker:PhantomData,
        }
    }
    #[inline]
    pub fn iter_mut(&mut self)->IterMut<'_,T>{
        IterMut{
            head:self.head,
            len:self.len,
            marker:PhantomData,
        }
    }
    #[inline]
    pub fn is_empty(&self)->bool{
        self.len==0
    }
    #[inline]
    pub fn len(&self)->usize{
        self.len
    }
    #[inline]
    pub fn clear(&mut self){
        *self=Self::new()
    }
    pub fn contains(&self,x:&T)->bool
    where
        T:std::cmp::PartialEq,
    {
        self.iter().any(|e| e==x)
    }
    #[inline]
    pub fn front(&self)->Option<&T>{
        unsafe{
            self.head.as_ref().map(|node| &node.as_ref().element)
        }
    }
    #[inline]
    pub fn front_mut(&mut self)->Option<&mut T>{
        unsafe{
            self.head.as_mut().map(|node| &mut node.as_mut().element)
        }
    }
    pub fn push_front(&mut self,element:T){
        self.push_front_node(Box::new(Node::new(element)));
    }
    pub fn pop_front(&mut self)->Option<T>{
        self.pop_front_node().map(Node::into_element)
    }
    pub fn split_off(&mut self,at:usize)->List<T>{
        let len=self.len();
        //handle special case if index is the start or end of list
        if at==0{
            return std::mem::take(self)
        }else if at==len{
            return Self::new()
        }
        let mut iter=self.iter_mut();
        for _ in 0..at-1{
            iter.next();
        }
        let my_tail=iter.head;
        let new_head=unsafe{
            (*my_tail.unwrap().as_ptr()).next
        };
        let new_len=self.len-at;
        unsafe{
            (*my_tail.unwrap().as_ptr()).next=None
        };
        unsafe{
            (*new_head.unwrap().as_ptr()).prev=None
        };
        self.len=at;
        Self{
            head:new_head,
            len:new_len,
            marker:self.marker,
        }
    }
    pub fn remove(&mut self,at:usize)->T{
        let len=self.len();
        assert!(at<len,"cannot remove at an index outside of list bounds");
        //special case for front
        if at==0{
            return self.pop_front().unwrap();
        }
        //find node to remove,from front or from back decending on index
        let remove={
            let mut remove=self.head.unwrap();
            for _ in 0..at{
                unsafe{
                    remove=(*remove.as_ptr()).next.unwrap();
                }
            }
            remove
        };
        //remap pointers to remove node from list
        unsafe{
            let remove=Box::from_raw(remove.as_ptr());
            if let Some(prev)=remove.prev{
                (*prev.as_ptr()).next=remove.next;
            }
            if let Some(next)=remove.next{
                (*next.as_ptr()).prev=remove.prev;
            }
            self.len-=1;
            remove.element
        }
    
    }
}
impl<'a,T> Iterator for Iter<'a,T>{
    type Item=&'a T;
    #[inline]
    fn next(&mut self)->Option<&'a T>{
        if self.len==0{
            None
        }else{
            self.head.map(|node| unsafe{
                let node=&*node.as_ptr();
                self.len-=1;
                self.head=node.next;
                &node.element
            })
        }
    }
    #[inline]
    fn size_hint(&self)->(usize,Option<usize>){
        (self.len,Some(self.len))
    }
}
impl<'a,T> Iterator for IterMut<'a,T>{
    type Item=&'a mut T;
    #[inline]
    fn next(&mut self)->Option<&'a mut T>{
        if self.len==0{
            None
        }else{
            self.head.map(|node| unsafe{
                let node=&mut *node.as_ptr();
                self.len-=1;
                self.head=node.next;
                &mut node.element
            })
        }
    }
    #[inline]
    fn size_hint(&self)->(usize,Option<usize>){
        (self.len,Some(self.len))
    }
}
pub struct IntoIter<T>{
    list:List<T>,
}
impl<T> Iterator for IntoIter<T>{
    type Item=T;
    #[inline]
    fn next(&mut self)->Option<T>{
        self.list.pop_front()
    }
    #[inline]
    fn size_hint(&self)->(usize,Option<usize>){
        (self.list.len,Some(self.list.len))
    }
}
impl<T> ExactSizeIterator for IntoIter<T>{}
impl<T> FusedIterator for IntoIter<T>{}
impl<T> IntoIterator for List<T>{
    type Item=T;
    type IntoIter=IntoIter<T>;
    #[inline]
    fn into_iter(self)->IntoIter<T>{
        IntoIter{
            list:self
        }
    }
}
impl<'a,T> IntoIterator for &'a List<T>{
    type Item=&'a T;
    type IntoIter=Iter<'a,T>;
    fn into_iter(self)->Iter<'a,T>{
        self.iter()
    }
}
impl<'a,T> IntoIterator for &'a mut List<T>{
    type Item=&'a mut T;
    type IntoIter=IterMut<'a,T>;
    fn into_iter(self)->IterMut<'a,T>{
        self.iter_mut()
    }
}
impl<T:PartialEq> PartialEq for List<T>{
    fn eq(&self,other:&Self)->bool{
        self.len()==other.len() && self.iter().eq(other)
    }
    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self,other:&Self)->bool{
        self.len()!=other.len() || self.iter().ne(other)
    }
}
impl<T:Eq> Eq for List<T>{}
impl<T:PartialOrd> PartialOrd for List<T>{
    fn partial_cmp(&self,other:&Self)->Option<Ordering>{
        self.iter().partial_cmp(other)
    }
}
impl<T:Ord> Ord for List<T>{
    #[inline]
    fn cmp(&self,other:&Self)->Ordering{
        self.iter().cmp(other)
    }
}
impl<T:Hash> Hash for List<T>{
    fn hash<H:Hasher>(&self,state:&mut H){
        self.len().hash(state);
        for elt in self{
            elt.hash(state);
        }
    }
}
unsafe impl<T: Send> Send for List<T> {}
unsafe impl<T: Sync> Sync for List<T> {}

unsafe impl<T: Sync> Send for Iter<'_, T> {}
unsafe impl<T: Sync> Sync for Iter<'_, T> {}

unsafe impl<T: Send> Send for IterMut<'_, T> {}
unsafe impl<T: Sync> Sync for IterMut<'_, T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn front() {
        let mut list = List::<u8>::new();
        list.push_front(0);
        assert_eq!(list.front(), Some(&0));
        list.push_front(1);
        assert_eq!(list.front(), Some(&1));
        list.clear();
        assert_eq!(list.front(), None);
    }

    #[test]
    fn pop_front() {
        let mut list = List::<u8>::new();
        list.push_front(0);
        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn push_front() {
        let mut list = List::<usize>::new();
        for v in 0..100 {
            assert_eq!(list.len(), v);
            list.push_front(v);
            assert_eq!(list.len(), v + 1);
        }
        for v in (0..100).rev() {
            assert_eq!(list.len(), v + 1);
            assert_eq!(list.pop_front(), Some(v));
            assert_eq!(list.len(), v);
        }
    }

    #[test]
    fn remove() {
        let mut list = List::<usize>::new();
        let items = 100;
        for start in 0..items {
            for v in (0..items).rev() {
                list.push_front(v);
            }
            for v in start..items {
                assert_eq!(list.remove(start), v);
            }
            list.clear();
        }
    }

    #[test]
    fn iter() {
        let mut list = List::<usize>::new();
        let items = 100;
        for v in (0..items).rev() {
            list.push_front(v);
        }
        let mut iter = list.iter();
        for v in 0..items {
            assert_eq!(iter.next(), Some(&v));
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn split_off() {
        let mut a = List::<usize>::new();
        for v in (0..100).rev() {
            a.push_front(v);
        }
        let b = a.split_off(50);
        assert_eq!(a.len(), 50);
        assert_eq!(b.len(), 50);
        assert_eq!(a.front(), Some(&0));
        assert_eq!(b.front(), Some(&50));
    }
}
