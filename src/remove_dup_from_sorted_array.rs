pub fn remove(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() { return 0; }
    let len: usize = nums.len()-1 as usize;
    let idx: i32 = 0;
    for i in 0..len {
        if nums[idx as usize] != nums[i] {
            nums[(idx+1) as usize] = nums[i];
        }
        
    }
    return idx + 1;
}
fn main() {
    let mut a = vec![1,1,2];
    println!("{:?}", remove(&mut a));
}
