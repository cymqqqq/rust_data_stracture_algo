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
//max depth BFS
pub fn max_depth(root:Option<Rc<RefCell<TreeNode>>>)->i32{
    if root.is_none() { return 0; }
    let mut depth = 0;
    let mut deque : VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
    deque.push_back(root);
    while !deque.is_empty() {
        let level_size = deque.len();
        let mut added = false;
        depth += 1;
        for _i in 0..level_size {
            added = true;
            if let Some(Some(node)) = deque.pop_front() {
                if node.borrow().left.is_some() { deque.push_back(node.borrow().left.clone());}
                if node.borrow().right.is_some() { deque.push_back(node.borrow().right.clone()); }
                
            }
        }
        if !added { break; }
        
    }
    depth
}

//max depth DFS
pub fn max_depth_dfs(root:Option<Rc<RefCell<TreeNode>>>)->i32{
    match root {
        Some(node) => {
            let left = max_depth_dfs(node.borrow().left.clone());
            let right = max_depth_dfs(node.borrow().right.clone());
            1 + left.max(right)
        },
        _ => 0,
    }
}
