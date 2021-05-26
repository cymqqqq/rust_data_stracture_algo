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
pub fn has_cycle(head:Option<Box<ListNode>>)->bool{
    let mut fast_p=&head;
    let mut slow_p=&head;
    while fast_p.is_some() && fast_p.as_ref().unwrap().next.is_some(){
        slow_p=&slow_p.as_ref().unwrap().next;
        fast_p=&fast_p.as_ref().unwrap().next.as_ref().unwrap().next;
        if slow_p==fast_p {return true;}
    }
    false
}
pub fn merge_two_list(l1:Option<Box<ListNode>>,l2:Option<Box<ListNode>>)->Option<Box<ListNode>>{
    match(l1,l2){
        (Some(node1),None)=>Some(node1),
        (None,Some(node2))=>Some(node2),
        (Some(mut node1),Some(mut node2))=>{
            if node1.val<node2.val{
                let n=node1.next.take();
                node1.next=merge_two_list(n,Some(node2));
                Some(node1)
            }else{
                let n=node2.next.take();
                node2.next=merge_two_list(Some(node1),n);
                Some(node2)
            }
        },
        _=>None,
    }
}
fn main(){
   println!("{:?}", merge_two_lists(to_list(vec![1, 3, 4]), to_list(vec![1, 2, 4])));

}
