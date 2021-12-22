pub fn is_sorted(arr: &mut Vec<isize>) -> bool {
    for i in 0..arr.len() {
        if i == arr.len() - 1 {
            break;
        }
        if arr[i] > arr[i + 1] {
            return false;
        }
    }
    return true;
}
