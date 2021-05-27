#[derive(Debug)]
pub struct ArrayStack{
    data:Vec<i32>,
    top:i32,
}
impl ArrayStack{
    fn new()->Self{
        ArrayStack{
            data:Vec::with_capacity(32),top:-1
        }
    }
    fn push(&mut self,value:i32){
        self.top+=1;
        if self.top==self.data.capacity() as i32{
            let temp_arr=self.data.clone();
            self.data=Vec::with_capacity(self.data.capacity()*2);
            for i in temp_arr.into_iter(){
                self.data.push(i);
            }
        }
        self.data.push(value);
    }
    fn pop(&mut self){
        if self.is_empty(){return;}
        self.top-=1;
        self.data.pop();
    }
    fn top(&self)->i32{
        if self.is_empty(){return -1;}
        *self.data.last().unwrap()
    }
    fn is_empty(&self)->bool{
        if self.top<0 {true} else {false}
    }
}
fn main(){
    let mut stack=ArrayStack::new();
    stack.push(-1);
    println!("{:?}",stack.top());
}
