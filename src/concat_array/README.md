# concat-arrays: a rust macro for concatenating fixed-size arrays
This crate defines concat_arrays!, a macro that allows you to concatenate arrays.
use concat_arrays::concat_arrays;

fn main() {
    let x = [0];
    let y = [1, 2];
    let z = [3, 4, 5];
    let concatenated = concat_arrays!(x, y, z);
    assert_eq!(concatenated, [0, 1, 2, 3, 4, 5]);
}

