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
pub fn reverse(head:Option<Box<ListNode>>)->Option<Box<ListNode>>{
    let mut prev=None;
    let mut cur=head;
    while let Some(mut boxed_node)=cur.take(){
        let next=boxed_node.next.take();
        prev=Some(boxed_node);
        cur=next;
    }
    prev
}
fn main(){
   // println!("{:?}", merge_two_lists(to_list(vec![1, 3, 4]), to_list(vec![1, 2, 4])));
       println!("{:?}", reverse(to_list(vec![1, 3, 4])));

}
