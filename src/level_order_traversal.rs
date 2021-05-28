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
pub fn level_order(root:Option<Rc<RefCell<TreeNode>>>)->Vec<Vec<i32>>{
    let mut result:Vec<Vec<i32>>=vec![];
    if root.is_none() {return result;}
    let mut deque:VecDeque<Option<Rc<RefCell<TreeNode>>>>=VecDeque::new();
    deque.push_back(root);
    while !deque.is_empty(){
        let mut current_level=vec![];
        let mut added=false;
        let level_size=deque.len();
        for i in 0..level_size{
            let n = deque.pop_front();
            if let Some(Some(node)) = n{
                current_level.push(node.borrow().val);
                added=true;
                if node.borrow().left.is_some(){ deque.push_back(node.borrow().left.clone());}
                if node.borrow().right.is_some() { deque.push_back(node.borrow().right.clone());}
            
            }
        }
        if !added {break;}
        result.push(current_level);
    }
    result
}
