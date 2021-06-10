
pub fn pow(x: i32) -> i32 {
    return x * x;
}
pub fn square(nums: &mut Vec<i32>) -> Vec<i32> {
    let mut start: usize = 0;
    let mut nums1: Vec<i32> = Vec::new();
    while start <= nums.len() - 1 {
        nums1.push(pow(nums[start]));
        start += 1;
    }
    for i in 0..nums1.len() - 1 {
        let j = i + 1;
        if nums1[i] > nums1[j] {
            let tmp = nums1[i];
            nums1[i] = nums1[j];
            nums1[j] = tmp;
        }
    }
    nums1
}
