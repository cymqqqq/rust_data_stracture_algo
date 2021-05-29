use std::collections::BinaryHeap;
pub fn get_top_k(nums: &mut Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let nums_len = nums.len() as i32;
    if nums_len <= k { return nums.clone(); }
    let mut heap = BinaryHeap::new();
    for _i in 0..k{
        heap.push(-nums[k as usize]);
    }
    
    for i in k + 1..nums_len {
        if -nums[i as usize] < *heap.peek().unwrap() {
            heap.pop();
            heap.push(-nums[i as usize]);
        }
    }
    nums.push(x);
    if -x < *heap.peek().unwrap() {
        heap.pop();
        heap.push(-x);
    }
    heap.iter().map(|h| h * -1).collect::<Vec<i32>>()
}

fn main() {
    let mut nums = vec![4, 5, 7, 9, 10, 6, 11];
    let m = get_top_k(&mut nums, 3, 23);
    println!("{:?}", m);
}
