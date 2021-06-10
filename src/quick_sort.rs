
pub fn swap(mut a: i32, mut b: i32) {
    let tmp = a;
    a = b;
    b = tmp;
}
//this function takes last element as pivot, places the pivot element at its
//correct position in sorted array, and places all smaller to left of pivot and all greater to right
pub fn partition( arr: Vec<i32>, low: i32, high: i32) -> i32 {
    let pivot = arr[high as usize]; //pivot
    let mut i = low - 1i32; //index of smaller element and indicates the right position
    for j in low..=high - 1 {
        //if current element is smaller than the pivot
        if arr[j as usize] < pivot {
            i += 1;
            swap(arr[i as usize], arr[j as usize]);
        }
    }
    swap( arr[(i + 1) as usize], arr[high as usize]);
    return i + 1;
}
//the main function that implement quicksort
pub fn quicksort(arr: &Vec<i32>, low: i32, high: i32) {
    if low < high {
        //pi is partitioning index, arr[p] is now at right place
        let pi = partition(arr.to_vec(), low, high);
        //sort element before partition and after partition
        quicksort(&arr, low, pi - 1);
        quicksort(&arr, pi + 1, high);
    }
}
pub fn printarr(arr: Vec<i32>) {
        println!("{:?}", arr);
}
