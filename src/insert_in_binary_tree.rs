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
pub fn insert_into_bst(root:Option<Rc<RefCell<TreeNode>>>,val:i32)->Option<Rc<RefCell<TreeNode>>>{
    insert(&root,val);
    root
}
fn insert(node:&Option<Rc<RefCell<TreeNode>>>,val:i32){
    if let Some(n) = node {
        let mut n = n.borrow_mut();
        let target = if val > n.val { &mut n.right } else { &mut n.left };
        if target.is_some() {
            return insert(target,val);
        }
        *target=Some(Rc::new(RefCell::new(TreeNode::new(val))));
    }
}
