
pub fn remove_dup(nums: &mut Vec<i32>, val: i32) -> &mut Vec<i32> {
    let mut start: usize = 0;
    while start < nums.len() {
        if nums[start] == val {
            nums.remove(start);
        }
        start += 1;
    }
    nums
}
