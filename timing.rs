use super::is_sorted::is_sorted;
use std::time::Instant;

pub fn quick_timing(original_arr: &mut Vec<isize>, algo: fn(arr: &mut Vec<isize>)) -> u128 {
    let mut copy_arr: Vec<isize> = original_arr.to_vec();
    let now = Instant::now();
    algo(&mut copy_arr);
    let elapsed = now.elapsed();
    assert!(is_sorted(&mut copy_arr));
    return elapsed.as_nanos();
}
pub fn merge_timing(
    original_arr: &mut Vec<isize>,
    algo: fn(arr: &mut Vec<isize>, low: usize, high: usize),
) -> u128 {
    let mut copy_arr: Vec<isize> = original_arr.to_vec();
    let last_index = copy_arr.len() - 1;
    let now = Instant::now();
    algo(&mut copy_arr, 0, last_index);
    let elapsed = now.elapsed();
    assert!(is_sorted(&mut copy_arr));
    return elapsed.as_nanos();
}
pub fn heap_timing(
    original_arr: &mut Vec<isize>,
    algo: fn(arr: &mut Vec<isize>, size: usize),
) -> u128 {
    let mut copy_arr: Vec<isize> = original_arr.to_vec();
    let size = copy_arr.len();
    let now = Instant::now();
    algo(&mut copy_arr, size);
    let elapsed = now.elapsed();
    assert!(is_sorted(&mut copy_arr));
    return elapsed.as_nanos();
}
