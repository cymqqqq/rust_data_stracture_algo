//defintion for singly-linked list
#[derive(PartialEq,Eq,Clone,Debug)]
pub struct ListNode{
    pub val:i32,
    pub next:Option<Box<ListNode>>
}
impl ListNode{
    #[inline]
    fn new(val:i32)->Self{
        ListNode{
            next:None,
            val
        }
    }
}
pub fn to_list(vec:Vec<i32>)->Option<Box<ListNode>>{
    let mut current=None;
    for &v in vec.iter().rev(){
        let mut node=ListNode::new(v);
        node.next=current;
        current=Some(Box::new(node));
    }
    current
}
#[macro_export]
macro_rules! linked{
    ($($e:expr),*)=>{to_list(vec![$($e.to_owned()),*])};
    ($($e:expr),*)=>{to_list(vec![$($e.to_owned()),*])};
}


pub fn middle_node(head:Option<Box<ListNode>>)->Option<Box<ListNode>>{
    let mut fast_p=&head;
    let mut slow_p=&head;
    while fast_p.is_some()&& fast_p.as_ref().unwrap().next.is_some(){
        slow_p=&slow_p.as_ref().unwrap().next;
        fast_p=&fast_p.as_ref().unwrap().next.as_ref().unwrap().next;
    }
    slow_p.clone()
}
fn main(){
    println!("{:?}", middle_node(to_list(vec![1, 3, 4])));

}
