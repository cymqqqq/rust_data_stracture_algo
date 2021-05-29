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
pub fn k_bag1(items: Vec<(i32, i32)>, capacity: i32) -> i32 {
    let mut states = vec![-1; (capacity + 1) as usize];
    let mut result = vec![];
    states[0] = 0;
    if items[0].0 <= capacity { states[items[0].0 as usize] = items[0].1; }
    for i in 1..items.len() {
        for j in 0..=(capacity - items[i].0) as usize {
            if states[j] >= 0 {
                let value = states[j] + items[i].1;
                if value > states[j+items[i].0 as usize] {
                    states[j+items[i].0 as usize] =  value;
                    result.push(items[i].0);
                }
            }
        }
    }
    let mut max_value = -1;
    for i in (0..=capacity as usize).rev() {
        if states[i] >= max_value {
            max_value= states[i];
        }
    }
    max_value
}
fn main() {
    let items = vec![2, 2, 4, 6, 3];
    let capacity = 9;
    let m = k_bag(items, capacity);
    println!("{}", m);
    let n = k_bag1(items, capacity);
    println!("{}", n);
}
