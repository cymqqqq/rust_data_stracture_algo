use std::rc::Rc;
use std::cell::RefCell;
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
pub fn inorder_traversal(root:Option<Rc<RefCell<TreeNode>>>)->Vec<i32>{
    let mut result:Vec<i32>=vec![];
    if root.is_none() { return result; }
    _inorder(root,&mut result);
    result
}
fn _inorder(root:Option<Rc<RefCell<TreeNode>>>,result:&mut Vec<i32>){
    match root{
        Some(node)=>{
            _inorder(node.borrow().left.clone(),result);
            result.push(node.borrow().val);
            _inorder(node.borrow().right.clone(),result);
        },
        None=>{return;}
    }
}
pub fn inorder_traversal1(root:Option<Rc<RefCell<TreeNode>>>)->Vec<i32>{
    let mut result=vec![];
    if root.is_none() {return result;}
    let mut stack:Vec<Rc<RefCell<TreeNode>>>=Vec::new();
    let mut r=root.clone();
    while r.is_some() || !stack.is_empty(){
        while let Some(node)=r{
            stack.push(node.clone());
            r=node.borrow().left.clone();
        }
        r=stack.pop();
        if let Some(node)=r{
            result.push(node.borrow().val);
            r=node.borrow().right.clone();
        }
    }
    result
}
