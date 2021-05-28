pub fn binary_search(nums:Vec<i32>,value:i32)->i32{
    if nums.is_empty(){return -1;}
    let mut low=0;
    let mut high=nums.len()-1;
    while low<=high{
        let mid=low+((high-low)>>1);
        if nums[mid]==value{return mid as i32;}
        if nums[mid]<value{
            low=mid+1;
        }else{
            low=mid-1;
        }
    }
    -1
}
fn main() {
    let nums1 = vec![8,11,19,23,27,33,45,55,67,98];
    println!("{:?}", binary_search(nums1, 23));
}
