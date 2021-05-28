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
//find the first given value
pub fn find_first_eq(nums:Vec<i32>,value:i32)->i32{
    if nums.is_empty(){return -1;}
    let mut start=0;
    let mut end=nums.len()-1;
    while start<=end{
        let mid=start+((end-start)>>1);
        if nums[mid]<=value{
            if mid==0||(nums[mid]==value&& nums[mid+1]!=value){return mid as i32;}
            start=mid+1;
        }else{
            end=mid-1;
        }
    }
    -1
}
//find the last given value
pub fn find_last_eq(nums:Vec<i32>,value:i32)->i32{
    if nums.is_empty(){return -1;}
    let mut start=0;
    let mut end=nums.len()-1;
    while start<=end{
        let mid=start+((end-start)>>1);
        if nums[mid]<=value{
            if mid==end||(nums[mid]==value&& nums[mid+1]!=value){return mid as i32;}
            start=mid+1;
        }else{
            end=mid-1;
        }
    }
    -1
}
//find the last elem, which is less than the given value
pub fn find_last_le(nums:Vec<i32>,value:i32)->i32{
    if nums.is_empty(){return -1;}
    let mut start=0;
    let mut end=nums.len()-1;
    while start<=end{
        let mid=start+((end-start)>>1);
        if nums[mid]<=value{
            if mid==0||(nums[mid]<=value&&nums[mid+1]>value){return mid as i32}
            start=mid+1;
        }else{
            end=mid-1;
        }
    }
    -1
}
fn main() {
    let nums1 = vec![1, 3, 5, 6, 8, 8, 8, 11, 18];
    let first_eq = find_first_eq(nums1, 8);
    println!("{:?}", first_eq);

    let nums2 = vec![1, 3, 5, 6, 8, 8, 8, 11, 18];
    let last_eq = find_last_eq(nums2, 8);
    println!("{:?}", last_eq);



    let nums4 = vec![1, 3, 5, 6, 8, 8, 8, 11, 18];
    let find_last_le = find_last_le(nums4, 17);
    println!("{:?}", find_last_le);

    let nums1 = vec![8,11,19,23,27,33,45,55,67,98];
    println!("{:?}", binary_search(nums1, 23));
}
