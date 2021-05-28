use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
#[derive(Debug,PartialEq,Eq)]
pub struct TreeNode{
    pub val: i32,
    pub left : Option<Rc<RefCell<TreeNode>>>,
    pub right : Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode{
    #[inline]
    pub fn new(val:i32)->Self{
        TreeNode{
            val,
            left:None,
            right:None,
        }
    }
}
pub fn to_tree(vec:Vec<Option<i32>>)->Option<Rc<RefCell<TreeNode>>>{
    use std::collections::VecDeque;
    let head=Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
    let mut queue=VecDeque::new();
    queue.push_back(head.as_ref().unwrap().clone());
    for child in vec[1..].chunks(2){
        let parent=queue.pop_front().unwrap();
        if let Some(v)=child[0]{
            parent.borrow_mut().left=Some(Rc::new(RefCell::new(TreeNode::new(v))));
            queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
        }
        if child.len()>1{
            if let Some(v)=child[1]{
                parent.borrow_mut().right=Some(Rc::new(RefCell::new(TreeNode::new(v))));
                
                queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }
    }
    head
}
#[macro_export]
macro_rules! tree{
    ()=>{
        None
    };
    ($($e:expr),*)=>{
        {
            let vec=vec![$(stringify!($e)),*];
            let vec=vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            to_tree(vec)
        }
    };
    ($($e:expr,)*)=>{(tree![$($e),*])};
}
pub fn postorder_traversal_recursive(root:Option<Rc<RefCell<TreeNode>>>)->Vec<i32>{
    let mut result:Vec<i32>=vec![];
    if root.is_none() { return result; }
    _postorder(root,&mut result);
    result
}
fn _postorder(root:Option<Rc<RefCell<TreeNode>>>,result:&mut Vec<i32>){
    match root{
        Some(node) => {
            _postorder(node.borrow().left.clone(),result);
            _postorder(node.borrow().right.clone(),result);
            result.push(node.borrow().val);
        },
        None => { return; }
    }
}
//iterating useing stack
pub fn postorder_traversal(root:Option<Rc<RefCell<TreeNode>>>)->Vec<i32>{
    let mut result=vec![];
    if root.is_none() { return result; }
    let mut stack1:Vec<Option<Rc<RefCell<TreeNode>>>>=Vec::new();
    let mut stack2:Vec<Option<Rc<RefCell<TreeNode>>>>=Vec::new();
    stack1.push(root);
    while let Some(Some(node)) = stack1.pop() {
        if node.borrow().left.is_some() {
            stack1.push(node.borrow().left.clone());
        }
        if node.borrow().right.is_some() {
            stack1.push(node.borrow().right.clone());
        }
        stack2.push(Some(node))
    }
    while let Some(Some(node)) = stack2.pop() {
        result.push(node.borrow().val);
    }
    result
}
