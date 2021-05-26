pub struct new_array{
    array:Vec<i32>,
    count:i32,
}
impl new_array{
    fn new(capacity:usize)->new_array{
        new_array{
            array:Vec::with_capacity(capacity),count:0
        }
    }
    fn find(&self,index:usize)->i32{
        if index>=self.count as usize {return -1;
            
        }
        self.array[index]
    }
    fn insert(&mut self,index:usize,value:i32)->bool{
        let array_count=self.count as usize;
        if index>array_count||array_count==self.array.capacity(){return false;}
        if index==array_count{
            self.array.push(value);
        }else{
            let temp_arr=self.array.clone();
            self.array=Vec::with_capacity(self.array.capacity());
            for i in 0..index{
                self.array.push(temp_arr[i]);
            }
            self.array.push(value);
            for i in index..array_count{
                self.array.push(temp_arr[i]);
            }
        }
        self.count+=1;
        true
    }
    fn remove(&mut self,index:usize)->i32{
        if index>=self.count as usize{return -1;}
        let result =self.array[index];
        let temp_arr=self.array.clone();
        self.array=Vec::with_capacity(self.array.capacity());
        for i in 0..index{
            self.array.push(temp_arr[i]);
        }
        for i in index+1..self.count as usize{
            self.array.push(temp_arr[i]);
        }
        self.count-=1;
        result
    }
}
fn main(){
    let mut new_array=new_array::new(5);
    assert_eq!(new_array.insert(0,3),true);
    assert_eq!(new_array.insert(1,3),true);
    assert_eq!(new_array.insert(2,3),true);
    assert_eq!(new_array.insert(3,3),true);
    assert_eq!(new_array.insert(4,3),true);
}
