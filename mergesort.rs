fn merge(arr: &mut Vec<isize>, left_index: usize, mid_index: usize, right_index: usize) {
    //Creating left - mid vector, and mid+1 to right vector
    let mut left: Vec<isize> = arr[left_index..mid_index + 1].to_vec();
    let mut right: Vec<isize> = arr[mid_index + 1..right_index + 1].to_vec();
    //Reversing the values so the last element is the smallest
    left.reverse();
    right.reverse();
    for i in left_index..right_index + 1 {
        //Iterating over the array checking if one of the vectors is empty, and if it is, 
        //inserting all the leftovers of the other vector into the array.
        if left.is_empty() {
            arr[i] = right.pop().unwrap();
            continue;
        }
        if right.is_empty() {
            arr[i] = left.pop().unwrap();
            continue;
        }
        //Iterating over the array inserting the smallest value of both vectors first
        if right.last() < left.last() {
            arr[i] = right.pop().unwrap();
        } else {
            arr[i] = left.pop().unwrap();
        }
    }
}

pub fn merge_sort(arr: &mut Vec<isize>, left_index: usize, right_index: usize) {
    //If it is possilbe to divide the array then merge_sort and merge it.
    if left_index < right_index {
        let mid_index = (left_index + right_index) / 2;
        merge_sort(arr, left_index, mid_index);
        merge_sort(arr, mid_index + 1, right_index);
        merge(arr, left_index, mid_index, right_index);
    }
}
