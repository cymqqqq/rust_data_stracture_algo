pub fn k_bag(item: Vec<i32>, capacity: i32) -> i32 {
    let mut states = vec![vec![false; (capacity + 1) as usize]; item.len()];
    let mut result = vec![];
    states[0][0] = true;
    if item[0] <= capacity { states[0][item[0] as usize] = true; }
    for i in 1..item.len() {
        for j in 0..=capacity as usize {
            if states[i-1][j] { states[i][j] = true; }
        }
        for j in 0..=(capacity - item[i]) as usize {
            if states[i-1][j] { states[i][j + item[i] as usize] = true; }
        }
    }
    let mut idx = capacity;
    while idx <= capacity {
        if states[item.len()-1][idx as usize] { break; }
        idx += 1;
    }
    for i in (1..item.len()).rev() {
        if idx - item[i] >= 0 && states[i-1][(idx-item[i]) as usize] {
            idx -= item[i];
            result.push(item[i]);
        }
    }
    if idx != 0 { result.push(item[0]); }
    println!("{:?}", result);
    for i in (0..=capacity as usize).rev() {
        if states[item.len()-1][i] { return i as i32; }
    }
    0
}
fn main() {
    let items = vec![2, 2, 4, 6, 3];
    let capacity = 9;
    let m = k_bag(items, capacity);
    println!("{}", m);
}
