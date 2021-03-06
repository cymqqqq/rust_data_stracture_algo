pub fn kmp_search(primary: &str, pattern: &str) -> Vec<i32> {
    if primary.is_empty() || pattern.is_empty() || pattern.len() > primary.len() { return vec![0]; }
    let primary_chars: Vec<char> = primary.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let max_match_lengths = get_failure_function(pattern);
    let mut count = 0;
    let m = pattern.len();
    let mut position = vec![];
    for i in 0..primary.len() {
        while count > 0 && pattern_chars[count as usize] != primary_chars[i] {
            count = max_match_lengths[(count-1) as usize];
        }
        if pattern_chars[count as usize] == primary_chars[i] {
            count += 1;
        }
        if count as usize == m {
            position.push((i - m + 1) as i32);
            count = max_match_lengths[(count - 1) as usize];
        }
    }
    position
}
pub fn get_failure_function(pattern: &str) -> Vec<i32> {
    let m = pattern.len();
    let mut max_match_length: Vec<i32> = vec![0; m];
    let mut max_length: i32 = 0;
    let pattern_char: Vec<char> = pattern.chars().collect();
    for i in 1..m {
        while max_length > 0 && pattern_char[max_length as usize] != pattern_char[i] {
            max_length = max_match_length[(max_length - 1) as usize];
        }
        if pattern_char[i] == pattern_char[max_length as usize] {
            max_length += 1;
        }
        max_match_length[i] = max_length;
    }
    max_match_length
}

fn main() {
    let primary1 = "abbaabbaaba";
    let pattern1 = "abbaaba";
    println!("{:?}", kmp_search(primary1, pattern1)); // 4

    let primary = "abc abcdab abcdabcdabde";
    let pattern = "bcdabd";
    println!("{:?}", kmp_search(primary, pattern)); // 16
}
