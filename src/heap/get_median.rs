use std::collections::BinaryHeap;
fn get_median(nums: &mut Vec<i32>, x: i32) -> i32 {
    let nums_len = nums.len();
    let mid = nums_len >> 1;
    let mut max_heap = BinaryHeap::new();
    let mut min_heap = BinaryHeap::new();
    nums.sort();
    for i in 0..nums_len {
        if i < mid {
            max_heap.push(nums[i]);
        } else {
            min_heap.push(nums[i]);
        }
    }
    nums.push(x);
    if x <= *max_heap.peek().unwrap() {
        max_heap.push(x);
    } else {
        min_heap.push(-x);
    }
    if max_heap.len() > min_heap.len() {
        min_heap.push(-max_heap.pop().unwrap());
    } else if min_heap.len() - max_heap.len() >= 2 {
        max_heap.push(-min_heap.pop().unwrap());
    }
    -*min_heap.peek().unwrap()
}
fn main() {
    let mut nums = vec![12, 45, 30, 77, 5, 6, 7, 8];
    let m = get_median(&mut nums, 9);
    println!("{:?}", m); // 9
    let n = get_median(&mut nums, 20);
    println!("{:?}", n); // 12
    let h = get_median(&mut nums, 11);
    println!("{:?}", h); // 11
    let i = get_median(&mut nums, 10);
    println!("{:?}", i); // 11
}
