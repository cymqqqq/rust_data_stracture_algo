pub fn insertion_sort(mut nums:Vec<i32>)->Vec<i32>{
    if nums.is_empty(){return vec![];}
    for i in 1..nums.len(){
        let value=nums[i];
        let mut j=(i-1) as i32;
        while j>=0{
            if nums[j as usize]>value{
                nums[(j+1) as usize]=nums[j as usize];
            }else{
                break;
            }
            j-=1;
        }
        nums[(j+1) as usize]=value;
    }
    nums
}
fn main() {
    let nums = vec![4, 5, 6, 1, 2, 3];
    println!("{:?}", insertion_sort(nums));
}
