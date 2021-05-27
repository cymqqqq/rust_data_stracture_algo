fn bubble_sort(mut nums:Vec<i32>)->Vec<i32>{
    if nums.is_empty(){return vec![];}
    let n=nums.len();
    for i in 0..n{
        let mut swap=false;
        for j in 0..n-1{
            if nums[j]>nums[j+1]{
                swap=true;
                let tmp=nums[j];
                nums[j]=nums[j+1];
                nums[j+1]=tmp;
            }
        }
        if !swap{break;}
    }
    nums
}
fn main() {
    let nums = vec![4, 5, 6, 1, 2, 3];
    println!("{:?}", bubble_sort(nums));
}
