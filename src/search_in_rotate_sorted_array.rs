pub fn search(nums: &mut Vec<i32>, target: i32) -> i32 {
    let mut first: usize = 0;
    let mut last = nums.len() as usize;
    while first != last {
        let mut mid = (first + (last - first) >> 1) as usize;
        if nums[mid] == target { return mid as i32; }
        if nums[first] <= nums[mid] {
            if nums[first] <= target && target <= nums[mid] { last = mid;}
            else { first = mid + 1;}
        } else {
            if nums[mid] < target && target <= nums[last - 1] { first = mid + 1; }
            else { last = mid; }
        }
    }
    return -1;
}
fn main() {
    let mut a = vec![1,2,3,4,5,6];
    println!("{:?}", search(&mut a,4));
}
